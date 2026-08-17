import {
  AmbientLight,
  AxesHelper,
  Color,
  DataTexture,
  DirectionalLight,
  GridHelper,
  Mesh,
  MeshStandardMaterial,
  PerspectiveCamera,
  Scene,
  WebGLRenderer,
} from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";

import { IVisualModelViews } from "@/core/visuals/lib/visual-views";
import { DEFAULT_VISUAL_PREVIEW_SCENE_CONFIG, IVisualPreviewSceneConfig } from "@/core/visuals/scene/scene-config";
import { createCheckerTexture, createSubmeshGeometry } from "@/core/visuals/scene/VisualPreviewScene.utils";
import { Nullable } from "@/lib/types/general";

/** Radius assumed when a model reports no usable extent, so the camera and helpers still have a scale. */
const FALLBACK_RADIUS: number = 1;

/**
 * View state the toolbar owns and the scene applies.
 *
 * React holds the state, the scene stays a sink: it never reports view state back, so there is one source of truth and
 * no synchronisation between react and the scene graph. Distinct from the scene's configuration, which is chosen once
 * and describes how the preview looks rather than what the user is toggling.
 */
export interface IVisualPreviewViewOptions {
  isWireframe: boolean;
  isGridVisible: boolean;
  isAxesVisible: boolean;
  /**
   * Renders a repeating checkerboard from the uv buffer instead of a flat surface.
   *
   * Present before textures are: it is the only way to see that the v flip is right, which otherwise stays invisible
   * until textures land and come out mirrored.
   */
  isCheckerVisible: boolean;
}

/**
 * Owns the three.js scene imperatively, outside of react state.
 *
 * An editor scene graph is long lived and mutated by direct manipulation, so it is deliberately not expressed as react
 * elements: react only mounts it into a container and disposes it again. Everything webgl touches stays behind this
 * class.
 */
export class VisualPreviewScene {
  private readonly config: IVisualPreviewSceneConfig;
  private readonly scene: Scene;
  private readonly camera: PerspectiveCamera;
  private readonly renderer: WebGLRenderer;
  private readonly controls: OrbitControls;
  private readonly material: MeshStandardMaterial;
  private readonly checker: DataTexture;
  private readonly grid: GridHelper;
  private readonly axes: AxesHelper;
  private readonly resizeObserver: ResizeObserver;

  private meshes: Array<Mesh> = [];
  private model: Nullable<IVisualModelViews> = null;
  private container: Nullable<HTMLElement> = null;
  private frameHandle: number = 0;
  private isResizePending: boolean = false;
  private renderedWidth: number = 0;
  private renderedHeight: number = 0;

  public constructor(
    model: Nullable<IVisualModelViews>,
    config: IVisualPreviewSceneConfig = DEFAULT_VISUAL_PREVIEW_SCENE_CONFIG
  ) {
    this.config = config;

    this.renderer = new WebGLRenderer({ antialias: true });
    this.renderer.setPixelRatio(window.devicePixelRatio);
    this.renderer.domElement.style.display = "block";

    this.scene = new Scene();
    this.scene.background = new Color(config.backgroundColor);

    this.camera = new PerspectiveCamera(config.cameraFieldOfView, 1, 0.001, 10000);

    this.controls = new OrbitControls(this.camera, this.renderer.domElement);
    this.controls.enableDamping = true;

    this.checker = createCheckerTexture(config);
    this.material = new MeshStandardMaterial({ color: config.meshColor, metalness: 0.05, roughness: 0.75 });
    this.grid = new GridHelper(10, 10, config.gridColor, config.gridColor);
    this.axes = new AxesHelper(1);

    const light: DirectionalLight = new DirectionalLight(0xffffff, 2);

    light.position.set(3, 5, 4);

    this.scene.add(new AmbientLight(0xffffff, 1.4));
    this.scene.add(light);
    this.scene.add(this.grid);
    this.scene.add(this.axes);

    this.resizeObserver = new ResizeObserver(() => this.resize());

    this.setModel(model);
  }

  /**
   * Replace whatever is on screen with a different model, or with nothing.
   *
   * Geometry is rebuilt while the renderer, camera and controls survive, so opening one visual after another neither
   * rebuilds the webgl context nor loses the current orbit.
   */
  public setModel(model: Nullable<IVisualModelViews>): void {
    for (const mesh of this.meshes) {
      this.scene.remove(mesh);
      mesh.geometry.dispose();
    }

    this.model = model;
    this.meshes = (model?.submeshes ?? []).map((submesh) => {
      const mesh: Mesh = new Mesh(createSubmeshGeometry(submesh), this.material);

      mesh.name = submesh.label;

      return mesh;
    });

    for (const mesh of this.meshes) {
      this.scene.add(mesh);
    }

    this.applyScale();
    this.resetCamera();
  }

  public applyViewOptions(options: IVisualPreviewViewOptions): void {
    this.material.wireframe = options.isWireframe;
    this.material.map = options.isCheckerVisible ? this.checker : null;
    this.material.needsUpdate = true;
    this.grid.visible = options.isGridVisible;
    this.axes.visible = options.isAxesVisible;
  }

  /**
   * Frame the model from its measured extent.
   *
   * A constant distance cannot serve this viewer: loose visuals run from a pistol a few centimetres across to an actor
   * two metres tall, so resetting the camera re-fits rather than returning to a fixed point.
   */
  public resetCamera(): void {
    const { cameraFieldOfView, cameraFitMargin, cameraDirection } = this.config;

    const radius: number = this.model?.fit.radius ?? FALLBACK_RADIUS;
    const [x, y, z] = this.model?.fit.center ?? [0, 0, 0];
    const distance: number = (radius / Math.sin((cameraFieldOfView * Math.PI) / 360)) * cameraFitMargin;
    const length: number = Math.hypot(cameraDirection[0], cameraDirection[1], cameraDirection[2]);

    this.camera.position.set(
      x + (cameraDirection[0] / length) * distance,
      y + (cameraDirection[1] / length) * distance,
      z + (cameraDirection[2] / length) * distance
    );
    this.camera.near = Math.max(distance / 1000, 0.0001);
    this.camera.far = distance * 100;
    this.camera.updateProjectionMatrix();

    this.controls.target.set(x, y, z);
    this.controls.update();
  }

  public mount(container: HTMLElement): void {
    this.container = container;
    container.appendChild(this.renderer.domElement);

    this.resizeObserver.observe(container);
    this.resize();
    this.renderFrame();
  }

  public dispose(): void {
    cancelAnimationFrame(this.frameHandle);

    this.resizeObserver.disconnect();
    this.controls.dispose();

    for (const mesh of this.meshes) {
      mesh.geometry.dispose();
    }

    this.meshes = [];

    this.checker.dispose();
    this.material.dispose();
    this.renderer.dispose();
    this.renderer.domElement.remove();

    this.container = null;
  }

  /** Size the helpers to the model, so the grid reads as ground rather than as a backdrop. */
  private applyScale(): void {
    const radius: number = this.model?.fit.radius ?? FALLBACK_RADIUS;

    this.grid.scale.setScalar(radius / 2);
    this.axes.scale.setScalar(radius);
  }

  /**
   * Note a size change without acting on it.
   *
   * `setSize` clears the drawing buffer, and doing that in the observer callback can paint before the frame that
   * refills it. Recording the request and applying it immediately before the next render keeps both in one frame, and
   * needs no timer: the frame loop is already the rate limit.
   */
  private resize(): void {
    this.isResizePending = true;
  }

  private applyPendingResize(): void {
    if (!this.isResizePending || !this.container) {
      return;
    }

    const width: number = this.container.clientWidth;
    const height: number = this.container.clientHeight;

    if (!width || !height) {
      return;
    }

    this.isResizePending = false;

    if (width === this.renderedWidth && height === this.renderedHeight) {
      return;
    }

    this.renderedWidth = width;
    this.renderedHeight = height;

    this.camera.aspect = width / height;
    this.camera.updateProjectionMatrix();
    this.renderer.setSize(width, height);
  }

  private renderFrame(): void {
    this.frameHandle = requestAnimationFrame(() => this.renderFrame());

    this.applyPendingResize();

    this.controls.update();
    this.renderer.render(this.scene, this.camera);
  }
}

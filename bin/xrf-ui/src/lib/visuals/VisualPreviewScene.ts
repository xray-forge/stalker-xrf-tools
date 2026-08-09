import {
  AmbientLight,
  AxesHelper,
  BufferAttribute,
  BufferGeometry,
  Color,
  DirectionalLight,
  GridHelper,
  Mesh,
  MeshStandardMaterial,
  PerspectiveCamera,
  Scene,
  WebGLRenderer,
} from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";

import { Optional } from "@/core/types/general";
import { IVisualMeshData } from "@/lib/visuals/visual_mesh";

const BACKGROUND_COLOR: number = 0x353535;
const GRID_COLOR: number = 0x505050;
const MESH_COLOR: number = 0xb0a999;

const CAMERA_POSITION: Array<number> = [1.8, 1.4, 2.2];

/**
 * View state the toolbar owns and the scene applies.
 *
 * React holds the state, the scene stays a sink: it never reports view state back, so there is one
 * source of truth and no synchronisation between react and the scene graph.
 */
export interface IVisualPreviewViewOptions {
  isWireframe: boolean;
  isGridVisible: boolean;
  isAxesVisible: boolean;
}

function createGeometry(mesh: IVisualMeshData): BufferGeometry {
  const geometry: BufferGeometry = new BufferGeometry();

  geometry.setAttribute("position", new BufferAttribute(mesh.positions, 3));
  geometry.setAttribute("normal", new BufferAttribute(mesh.normals, 3));
  geometry.setAttribute("uv", new BufferAttribute(mesh.uvs, 2));
  geometry.setIndex(new BufferAttribute(mesh.indices, 1));
  geometry.computeBoundingSphere();

  return geometry;
}

/**
 * Owns the three.js scene imperatively, outside of react state.
 *
 * An editor scene graph is long lived and mutated by direct manipulation, so it is deliberately not
 * expressed as react elements: react only mounts it into a container and disposes it again. Everything
 * webgl touches stays behind this class.
 */
export class VisualPreviewScene {
  private readonly scene: Scene;
  private readonly camera: PerspectiveCamera;
  private readonly renderer: WebGLRenderer;
  private readonly controls: OrbitControls;
  private readonly geometry: BufferGeometry;
  private readonly material: MeshStandardMaterial;
  private readonly grid: GridHelper;
  private readonly axes: AxesHelper;
  private readonly resizeObserver: ResizeObserver;

  private container: Optional<HTMLElement> = null;
  private frameHandle: number = 0;

  public constructor(mesh: IVisualMeshData) {
    this.renderer = new WebGLRenderer({ antialias: true });
    this.renderer.setPixelRatio(window.devicePixelRatio);
    this.renderer.domElement.style.display = "block";

    this.scene = new Scene();
    this.scene.background = new Color(BACKGROUND_COLOR);

    this.camera = new PerspectiveCamera(50, 1, 0.01, 1000);

    this.controls = new OrbitControls(this.camera, this.renderer.domElement);
    this.controls.enableDamping = true;

    this.geometry = createGeometry(mesh);
    this.material = new MeshStandardMaterial({ color: MESH_COLOR, metalness: 0.05, roughness: 0.75 });
    this.grid = new GridHelper(10, 10, GRID_COLOR, GRID_COLOR);
    this.axes = new AxesHelper(1);

    const light: DirectionalLight = new DirectionalLight(0xffffff, 2);

    light.position.set(3, 5, 4);

    this.scene.add(new AmbientLight(0xffffff, 1.4));
    this.scene.add(light);
    this.scene.add(new Mesh(this.geometry, this.material));
    this.scene.add(this.grid);
    this.scene.add(this.axes);

    this.resizeObserver = new ResizeObserver(() => this.resize());

    this.resetCamera();
  }

  public applyViewOptions(options: IVisualPreviewViewOptions): void {
    this.material.wireframe = options.isWireframe;
    this.grid.visible = options.isGridVisible;
    this.axes.visible = options.isAxesVisible;
  }

  public resetCamera(): void {
    this.camera.position.set(CAMERA_POSITION[0], CAMERA_POSITION[1], CAMERA_POSITION[2]);
    this.controls.target.set(0, 0, 0);
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
    this.geometry.dispose();
    this.material.dispose();
    this.renderer.dispose();
    this.renderer.domElement.remove();

    this.container = null;
  }

  private resize(): void {
    if (!this.container) {
      return;
    }

    const width: number = this.container.clientWidth;
    const height: number = this.container.clientHeight;

    if (!width || !height) {
      return;
    }

    this.camera.aspect = width / height;
    this.camera.updateProjectionMatrix();
    this.renderer.setSize(width, height);
  }

  private renderFrame(): void {
    this.frameHandle = requestAnimationFrame(() => this.renderFrame());

    this.controls.update();
    this.renderer.render(this.scene, this.camera);
  }
}

import { default as ArchiveIcon } from "@mui/icons-material/Archive";
import { default as ForumIcon } from "@mui/icons-material/Forum";
import { default as ImageIcon } from "@mui/icons-material/Image";
import { default as MapIcon } from "@mui/icons-material/Map";
import { default as SettingsApplicationsIcon } from "@mui/icons-material/SettingsApplications";
import { default as SwapHorizIcon } from "@mui/icons-material/SwapHoriz";
import { default as TranslateIcon } from "@mui/icons-material/Translate";
import { default as ViewInArIcon } from "@mui/icons-material/ViewInAr";

import { ARCHIVES_APPLICATION } from "@/applications/archives/application";
import { ARCHIVES_UNPACK_APPLICATION } from "@/applications/archives-unpack/application";
import { CONFIGS_EXPLORER_APPLICATION } from "@/applications/configs-explorer/application";
import { CONFIGS_FORMAT_APPLICATION } from "@/applications/configs-format/application";
import { CONFIGS_VERIFY_APPLICATION } from "@/applications/configs-verify/application";
import { DESCRIPTION_ICONS_APPLICATION } from "@/applications/description-icons/application";
import { DESCRIPTION_ICONS_PACK_APPLICATION } from "@/applications/description-icons-pack/application";
import { DESCRIPTION_ICONS_UNPACK_APPLICATION } from "@/applications/description-icons-unpack/application";
import { DIALOGS_APPLICATION } from "@/applications/dialogs/application";
import { EQUIPMENT_ICONS_APPLICATION } from "@/applications/equipment-icons/application";
import { EQUIPMENT_ICONS_PACK_APPLICATION } from "@/applications/equipment-icons-pack/application";
import { EQUIPMENT_ICONS_UNPACK_APPLICATION } from "@/applications/equipment-icons-unpack/application";
import { EXPORTS_APPLICATION } from "@/applications/exports/application";
import { PROJECT_VISUALS_APPLICATION } from "@/applications/project-visuals/application";
import { SPAWN_APPLICATION } from "@/applications/spawn/application";
import { SPAWN_PACK_APPLICATION } from "@/applications/spawn-pack/application";
import { SPAWN_UNPACK_APPLICATION } from "@/applications/spawn-unpack/application";
import { TRANSLATIONS_APPLICATION } from "@/applications/translations/application";
import { VISUAL_PREVIEW_APPLICATION } from "@/applications/visual-preview/application";
import { EApplicationGroupId, IApplicationDescriptor, IApplicationGroup } from "@/core/routing/application";
import { Nullable } from "@/lib/types/general";

export interface IApplicationCatalog {
  readonly applications: ReadonlyArray<IApplicationDescriptor>;
  readonly groups: ReadonlyArray<IApplicationGroup>;
  findApplicationByPath(pathname: string): Nullable<IApplicationDescriptor>;
  findApplicationById(id: string): Nullable<IApplicationDescriptor>;
  findApplicationGroupById(id: string): Nullable<IApplicationGroup>;
}

export const APPLICATION_CATALOG: IApplicationCatalog = {
  applications: [
    ARCHIVES_APPLICATION,
    ARCHIVES_UNPACK_APPLICATION,
    CONFIGS_EXPLORER_APPLICATION,
    CONFIGS_VERIFY_APPLICATION,
    CONFIGS_FORMAT_APPLICATION,
    DIALOGS_APPLICATION,
    EXPORTS_APPLICATION,
    EQUIPMENT_ICONS_APPLICATION,
    EQUIPMENT_ICONS_PACK_APPLICATION,
    EQUIPMENT_ICONS_UNPACK_APPLICATION,
    DESCRIPTION_ICONS_APPLICATION,
    DESCRIPTION_ICONS_PACK_APPLICATION,
    DESCRIPTION_ICONS_UNPACK_APPLICATION,
    SPAWN_APPLICATION,
    SPAWN_PACK_APPLICATION,
    SPAWN_UNPACK_APPLICATION,
    TRANSLATIONS_APPLICATION,
    VISUAL_PREVIEW_APPLICATION,
    PROJECT_VISUALS_APPLICATION,
  ],
  groups: [
    { id: EApplicationGroupId.ARCHIVES, label: "Archives", icon: <ArchiveIcon /> },
    { id: EApplicationGroupId.CONFIGS, label: "Configs", icon: <SettingsApplicationsIcon /> },
    { id: EApplicationGroupId.DIALOGS, label: "Dialogs", icon: <ForumIcon /> },
    { id: EApplicationGroupId.EXPORTS, label: "Exports", icon: <SwapHorizIcon /> },
    { id: EApplicationGroupId.ICONS, label: "Icons", icon: <ImageIcon /> },
    { id: EApplicationGroupId.SPAWNS, label: "Spawns", icon: <MapIcon /> },
    { id: EApplicationGroupId.TRANSLATIONS, label: "Translations", icon: <TranslateIcon /> },
    { id: EApplicationGroupId.VISUALS, label: "Visuals", icon: <ViewInArIcon /> },
  ],
  findApplicationByPath: (pathname: string): Nullable<IApplicationDescriptor> =>
    APPLICATION_CATALOG.applications.find(
      (application: IApplicationDescriptor) =>
        pathname === application.path || pathname.startsWith(`${application.path}/`)
    ) ?? null,
  findApplicationById: (id: string): Nullable<IApplicationDescriptor> =>
    APPLICATION_CATALOG.applications.find((application: IApplicationDescriptor) => application.id === id) ?? null,
  findApplicationGroupById: (id: string): Nullable<IApplicationGroup> =>
    APPLICATION_CATALOG.groups.find((group: IApplicationGroup) => group.id === id) ?? null,
};

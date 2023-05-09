import type { ParamBase, RenderObjectDef } from "./def";

import type { Param } from "./param";
import type { ShaderId } from "./pipeline";

const store = new Map<string, Param>();
const renderObjectDefs = new Map<string, RenderObjectDef<ParamBase<symbol, unknown>[]>>();
const shaders = new Map<string, ShaderId>();

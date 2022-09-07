declare const shaderIdNominal: unique symbol;
export type ShaderId = number & { [shaderIdNominal]: never };

export interface Pipeline {
    loadShader(vertSource: string, fragSource: string): ShaderId;
    useShader(id: ShaderId): void;
}

export class WebGlPipeline implements Pipeline {
    private readonly ctx: WebGL2RenderingContext;
    private readonly shaders: WebGLProgram[] = [];

    constructor(elem: HTMLCanvasElement) {
        const ctx = elem.getContext("webgl2");
        if (!ctx) {
            throw new Error("failed to get webgl2 context");
        }
        this.ctx = ctx;
    }

    private compileShader(source: string, kind: number): WebGLShader {
        const shader = this.ctx.createShader(kind);
        if (!shader) {
            throw new Error("failed to create shader");
        }
        this.ctx.shaderSource(shader, source);
        this.ctx.compileShader(shader);
        if (!this.ctx.getShaderParameter(shader, this.ctx.COMPILE_STATUS)) {
            throw new Error(`${this.ctx.getShaderInfoLog(shader)}`);
        }
        return shader;
    }

    private linkShader(vert: WebGLShader, frag: WebGLShader): WebGLProgram {
        const program = this.ctx.createProgram();
        if (!program) {
            throw new Error("failed to create program");
        }

        this.ctx.attachShader(program, vert);
        this.ctx.attachShader(program, frag);
        this.ctx.linkProgram(program);

        if (!this.ctx.getProgramParameter(program, this.ctx.LINK_STATUS)) {
            throw new Error(`${this.ctx.getProgramInfoLog(program)}`);
        }
        return program;
    }

    loadShader(vertSource: string, fragSource: string): ShaderId {
        const vert = this.compileShader(vertSource, this.ctx.VERTEX_SHADER);
        const frag = this.compileShader(fragSource, this.ctx.FRAGMENT_SHADER);
        const program = this.linkShader(vert, frag);
        const id = this.shaders.length as ShaderId;
        this.shaders.push(program);
        return id;
    }

    useShader(id: ShaderId): void {
        this.ctx.useProgram(this.shaders[id]);
    }
}

precision mediump float;
in vec2 v_verts_uv;
out vec4 out_color;
uniform sampler2D u_sampler;
void main() {
    out_color = texture(u_sampler, v_verts_uv);
}
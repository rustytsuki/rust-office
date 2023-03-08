const vec2 verts_pos[6] = vec2[6](
    vec2(-1.0, 1.0),
    vec2(-1.0, -1.0),
    vec2(1.0, -1.0),
    vec2(1.0, -1.0),
    vec2(1.0, 1.0),
    vec2(-1.0, 1.0)
);

const vec2 verts_uv[6] = vec2[6](
    vec2(0.0, 0.0),
    vec2(0.0, 1.0),
    vec2(1.0, 1.0),
    vec2(1.0, 1.0),
    vec2(1.0, 0.0),
    vec2(0.0, 0.0)
);

out vec2 v_verts_uv;

void main() {
    v_verts_uv = verts_uv[gl_VertexID];
    gl_Position = vec4(verts_pos[gl_VertexID], 0.0, 1.0);
}
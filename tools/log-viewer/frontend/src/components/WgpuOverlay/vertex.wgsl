// vertex.wgsl — full-screen quad vertex shader
//
// Emits six vertices forming two triangles that cover the entire NDC quad
// (-1,-1) to (1,1).  No vertex buffer is needed; positions are derived from
// the built-in vertex index.

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> @builtin(position) vec4f {
    var pos = array<vec2f, 6>(
        vec2f(-1.0, -1.0), vec2f( 1.0, -1.0), vec2f(-1.0,  1.0),
        vec2f(-1.0,  1.0), vec2f( 1.0, -1.0), vec2f( 1.0,  1.0)
    );
    return vec4f(pos[vi], 0.0, 1.0);
}

#version 450
layout(location = 0) in vec3 v_Position;
layout(location = 1) in vec3 v_Normal;
layout(location = 2) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform WireframeMaterial_color {
    vec4 color;
};

void main(void) {
  # ifdef WIREFRAMEMATERIAL_DASHED
    int pattern = 0x3F3F;
    //int pattern = 0x6666;
    int nPatterns = 300;

    // use 4 bytes for the masking pattern
    // map the texture coordinate to the interval [0,2*8[
    uint bitpos = uint(round(v_Uv * nPatterns)) % 16U;
    // move a unit bit 1U to position bitpos so that
    // bit is an integer between 1 and 1000 0000 0000 0000 = 0x8000
    uint bit = (1U << bitpos);

    // test the bit against the masking pattern
    //  Line::SOLID:       pattern = 0xFFFF;  // = 1111 1111 1111 1111 = solid pattern
    //  Line::DASH:        pattern = 0x3F3F;  // = 0011 1111 0011 1111
    //  Line::DOT:         pattern = 0x6666;  // = 0110 0110 0110 0110
    //  Line::DASHDOT:     pattern = 0xFF18;  // = 1111 1111 0001 1000
    //  Line::DASHDOTDOT:  pattern = 0x7E66;  // = 0111 1110 0110 0110
    uint up = uint(pattern);

    // discard the bit if it doesn't match the masking pattern
    if ((up & bit) == 0U) discard;

    //o_Target = vec4(color.rgb, 1.0);
    o_Target = vec4(v_Uv, 0.0, 1.0);
  # else
    o_Target = vec4(v_Position, 1.0);
    //o_Target = vec4(color.x, color.y, color.z, 1.0);
  # endif
}
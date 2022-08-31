shader_type spatial;

uniform sampler2D text;

void fragment() {
	ALBEDO = texture(text, UV).rgb;
}
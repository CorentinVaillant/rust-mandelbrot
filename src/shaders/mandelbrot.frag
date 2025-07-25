#version 330

out vec4 fragColor;

uniform vec2 resolution;
uniform vec2 start;
uniform float palette_offset;

const int MAX_ITER = 1000;
const int Z_CACHE_SIZE = 10;

vec2 c_mult(vec2 z1, vec2 z2){
    return vec2(z1.x*z2.x - z1.y*z2.y, z1.x*z2.y + z2.x * z1.y);
}

vec2 c_sq(vec2 z){
    return c_mult(z, z);
}

vec2 c_pow3(vec2 z){
    return c_mult(z,c_mult(z, z));
}

vec2 z_n(vec2 z,vec2 c){
    return (c_sq(z) + c);
}

float smooth_iter(vec2 z, int i) {
    float log_zn = log(dot(z, z)) / 2.0;
    float nu = log(log_zn / log(2.0)) / log(2.0);
    return float(i) + 1.0 - nu;
}

vec3 palette(float t) {
    return 0.5 + 0.5 * cos(6.2831 * (vec3(0.0, 0.33, 0.67) + t + palette_offset));
}

vec4 mandelbrot(vec2 c){
    vec2 cache[Z_CACHE_SIZE];

    vec2 z = start;

    for (int i = 0; i < Z_CACHE_SIZE; i++) {
        cache[i] = vec2(0.);
    }

    // z² + C
    for (int i = 0; i < MAX_ITER; i++) {
        z = z_n(z,c);

        if (dot(z,z) >= 4.0){ //if the length is >= 2.
            float iter = smooth_iter(z, i);
            return vec4(palette(iter / float(MAX_ITER)), 1.0);
        }

        for (int j = 0; j < Z_CACHE_SIZE; j++) {
            if (distance(z, cache[j]) < 1e-6) {
                return vec4(0.0, 0.0, 0.0, 1.0); 
            }
        }

        if (i < Z_CACHE_SIZE) {
            cache[i] = z;
        }
    }

    return vec4(0.0, 0.0, 0.0, 1.0);

}

void main()
{
    float aspect_ratio = resolution.x / resolution.y;
    vec2 uv = (gl_FragCoord.xy / resolution.x - vec2(0.5,0.25)) * 3.;
    
    vec2 c = uv;
    
    fragColor = mandelbrot(c);
   
}

pub mod compute_shaders {
    pub mod pointwise_multiplication_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

                    layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

                    layout(set = 0, binding = 0) buffer Source {
                        float data[];
                    } src;

                    layout(set = 1, binding = 1) buffer Destentaion {
                        float data[];
                    } dest;


                    void main() {
                        uint idx = gl_GlobalInvocationID.x;
                        dest.data[idx] *= src.data[idx];
                    }
                ",
        }
    }

    pub mod convolution_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

                    layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

                    layout(set = 0, binding = 0) buffer Source1 {
                        float data[];
                    } src1;

                    layout(set = 1, binding = 1) buffer Source2 {
                        float data[];
                    } src2;

                    layout(set = 2, binding = 2) buffer Destentaion {
                        float data[];
                    } dest;

                    void main() {
                        for(int i = 0; i < src1.data.length() && i <= gl_GlobalInvocationID.x; i++){
                            dest.data[gl_GlobalInvocationID.x] += src1.data[i] * src2.data[gl_GlobalInvocationID.x - i];
                        }
                    }
                ",
        }
    }

    pub mod scalar_multiplication_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

                    layout(local_size_x = 32, local_size_y = 1, local_size_z = 1) in;

                    layout(set = 0, binding = 0) buffer Scalar {
                        float value;
                    } scal;

                    layout(set = 1, binding = 1) buffer Source {
                        float data[];
                    } dest;

                    void main() {
                        dest.data[gl_GlobalInvocationID.x] *= scal.value;
                    }
                ",
        }
    }

    pub mod sin_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

                    layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

                    layout(set = 0, binding = 0) buffer Source {
                        float data[];
                    } dest;

                    void main() {
                        dest.data[gl_GlobalInvocationID.x] = sin(dest.data[gl_GlobalInvocationID.x]);
                    }
                ",
        }
    }

    pub mod cos_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

                    layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

                    layout(set = 0, binding = 0) buffer Source {
                        float data[];
                    } dest;

                    void main() {
                        dest.data[gl_GlobalInvocationID.x] = cos(dest.data[gl_GlobalInvocationID.x]);
                    }
                ",
        }
    }

    pub mod mod_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

                    layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

                    layout(set = 0, binding = 0) buffer Scalar {
                        float value;
                    } scal;

                    layout(set = 1, binding = 1) buffer Source {
                        float data[];
                    } dest;

                    void main() {
                                                                                                                             // Fucking rounding errors
                        dest.data[gl_GlobalInvocationID.x] -= scal.value * floor((dest.data[gl_GlobalInvocationID.x] / scal.value) + 0.0001);
                    }
                ",
        }
    }
}



use half::f16;
use crate::matrices::{ColumnVector, MatrixCr};

/*
#define GL_FLOAT_MAT2                     0x8B5A	MatrixCr?<f32, 2, 2>
#define GL_FLOAT_MAT3                     0x8B5B	MatrixCr?<f32, 3, 3>
#define GL_FLOAT_MAT4                     0x8B5C	MatrixCr?<f32, 4, 4>
#define GL_FLOAT_MAT2x3                   0x8B65	MatrixCr?<f32, 2, 3>
#define GL_FLOAT_MAT2x4                   0x8B66	MatrixCr?<f32, 2, 4>
#define GL_FLOAT_MAT3x2                   0x8B67	MatrixCr?<f32, 3, 2>
#define GL_FLOAT_MAT3x4                   0x8B68	MatrixCr?<f32, 3, 4>
#define GL_FLOAT_MAT4x2                   0x8B69	MatrixCr?<f32, 4, 2>
#define GL_FLOAT_MAT4x3                   0x8B6A	MatrixCr?<f32, 4, 3>
#define GL_DOUBLE_MAT2                    0x8F46	MatrixCr?<f64, 2, 2>
#define GL_DOUBLE_MAT3                    0x8F47	MatrixCr?<f64, 3, 3>
#define GL_DOUBLE_MAT4                    0x8F48	MatrixCr?<f64, 4, 4>
#define GL_DOUBLE_MAT2x3                  0x8F49	MatrixCr?<f64, 2, 3>
#define GL_DOUBLE_MAT2x4                  0x8F4A	MatrixCr?<f64, 2, 4>
#define GL_DOUBLE_MAT3x2                  0x8F4B	MatrixCr?<f64, 3, 2>
#define GL_DOUBLE_MAT3x4                  0x8F4C	MatrixCr?<f64, 3, 4>
#define GL_DOUBLE_MAT4x2                  0x8F4D	MatrixCr?<f64, 4, 2>
#define GL_DOUBLE_MAT4x3                  0x8F4E	MatrixCr?<f64, 4, 3>
#define GL_FLOAT_MAT2_ARB                 0x8B5A	??
#define GL_FLOAT_MAT3_ARB                 0x8B5B
#define GL_FLOAT_MAT4_ARB                 0x8B5C
#define GL_FLOAT16_MAT2_AMD               0x91C5	MatrixCr?<f16, 2, 2>
#define GL_FLOAT16_MAT3_AMD               0x91C6	MatrixCr?<f16, 3, 3>
#define GL_FLOAT16_MAT4_AMD               0x91C7	MatrixCr?<f16, 4, 4>
#define GL_FLOAT16_MAT2x3_AMD             0x91C8	MatrixCr?<f16, 2, 3>
#define GL_FLOAT16_MAT2x4_AMD             0x91C9	MatrixCr?<f16, 2, 4>
#define GL_FLOAT16_MAT3x2_AMD             0x91CA	MatrixCr?<f16, 3, 2>
#define GL_FLOAT16_MAT3x4_AMD             0x91CB	MatrixCr?<f16, 3, 4>
#define GL_FLOAT16_MAT4x2_AMD             0x91CC	MatrixCr?<f16, 4, 2>
#define GL_FLOAT16_MAT4x3_AMD             0x91CD	MatrixCr?<f16, 4, 3>
#define GL_DOUBLE_MAT2_EXT                0x8F46	MatrixCr?<f64, 2, 2>
#define GL_DOUBLE_MAT3_EXT                0x8F47	MatrixCr?<f64, 3, 3>
#define GL_DOUBLE_MAT4_EXT                0x8F48	MatrixCr?<f64, 4, 4>
#define GL_DOUBLE_MAT2x3_EXT              0x8F49
#define GL_DOUBLE_MAT2x4_EXT              0x8F4A
#define GL_DOUBLE_MAT3x2_EXT              0x8F4B
#define GL_DOUBLE_MAT3x4_EXT              0x8F4C
#define GL_DOUBLE_MAT4x2_EXT              0x8F4D
#define GL_DOUBLE_MAT4x3_EXT              0x8F4E

#define GL_FLOAT_VEC2                     0x8B50	ColumnVector<f32, 2>
#define GL_FLOAT_VEC3                     0x8B51	ColumnVector<f32, 3>
#define GL_FLOAT_VEC4                     0x8B52	ColumnVector<f32, 4>
#define GL_INT_VEC2                       0x8B53
#define GL_INT_VEC3                       0x8B54
#define GL_INT_VEC4                       0x8B55
#define GL_BOOL_VEC2                      0x8B57
#define GL_BOOL_VEC3                      0x8B58
#define GL_BOOL_VEC4                      0x8B59
#define GL_UNSIGNED_INT_VEC2              0x8DC6
#define GL_UNSIGNED_INT_VEC3              0x8DC7
#define GL_UNSIGNED_INT_VEC4              0x8DC8
#define GL_DOUBLE_VEC2                    0x8FFC	ColumnVector<f64, 2>
#define GL_DOUBLE_VEC3                    0x8FFD	ColumnVector<f64, 3>
#define GL_DOUBLE_VEC4                    0x8FFE	ColumnVector<f64, 4>
#define GL_INT64_VEC2_ARB                 0x8FE9
#define GL_INT64_VEC3_ARB                 0x8FEA
#define GL_INT64_VEC4_ARB                 0x8FEB
#define GL_UNSIGNED_INT64_VEC2_ARB        0x8FF5
#define GL_UNSIGNED_INT64_VEC3_ARB        0x8FF6
#define GL_UNSIGNED_INT64_VEC4_ARB        0x8FF7
#define GL_FLOAT_VEC2_ARB                 0x8B50
#define GL_FLOAT_VEC3_ARB                 0x8B51
#define GL_FLOAT_VEC4_ARB                 0x8B52
#define GL_INT_VEC2_ARB                   0x8B53
#define GL_INT_VEC3_ARB                   0x8B54
#define GL_INT_VEC4_ARB                   0x8B55
#define GL_BOOL_VEC2_ARB                  0x8B57
#define GL_BOOL_VEC3_ARB                  0x8B58
#define GL_BOOL_VEC4_ARB                  0x8B59
#define GL_FLOAT16_VEC2_NV                0x8FF9
#define GL_FLOAT16_VEC3_NV                0x8FFA
#define GL_FLOAT16_VEC4_NV                0x8FFB
#define GL_INT8_VEC2_NV                   0x8FE1
#define GL_INT8_VEC3_NV                   0x8FE2
#define GL_INT8_VEC4_NV                   0x8FE3
#define GL_INT16_VEC2_NV                  0x8FE5
#define GL_INT16_VEC3_NV                  0x8FE6
#define GL_INT16_VEC4_NV                  0x8FE7
#define GL_INT64_VEC2_NV                  0x8FE9
#define GL_INT64_VEC3_NV                  0x8FEA
#define GL_INT64_VEC4_NV                  0x8FEB
#define GL_UNSIGNED_INT8_VEC2_NV          0x8FED
#define GL_UNSIGNED_INT8_VEC3_NV          0x8FEE
#define GL_UNSIGNED_INT8_VEC4_NV          0x8FEF
#define GL_UNSIGNED_INT16_VEC2_NV         0x8FF1
#define GL_UNSIGNED_INT16_VEC3_NV         0x8FF2
#define GL_UNSIGNED_INT16_VEC4_NV         0x8FF3
#define GL_UNSIGNED_INT64_VEC2_NV         0x8FF5
#define GL_UNSIGNED_INT64_VEC3_NV         0x8FF6
#define GL_UNSIGNED_INT64_VEC4_NV         0x8FF7
#define GL_UNSIGNED_INT_VEC2_EXT          0x8DC6
#define GL_UNSIGNED_INT_VEC3_EXT          0x8DC7
#define GL_UNSIGNED_INT_VEC4_EXT          0x8DC8
#define GL_DOUBLE_VEC2_EXT                0x8FFC
#define GL_DOUBLE_VEC3_EXT                0x8FFD
#define GL_DOUBLE_VEC4_EXT                0x8FFE

https://docs.rs/gles30/0.2.0/gles30/constant.GL_FLOAT_VEC3.html
	gles30

// https://cr-rev.appspot.com/f40bb190ece97c908f8dba2efc7c1aceb4fc0e0b/gpu_renderer/src/generated/epoxy_egl.rs

DECL_VEC_TYPE(bool,      bvec,   GLSL_TYPE_BOOL,    GL_BOOL)
DECL_VEC_TYPE(int,       ivec,   GLSL_TYPE_INT,     GL_INT)
DECL_VEC_TYPE(uint,      uvec,   GLSL_TYPE_UINT,    GL_UNSIGNED_INT)
DECL_VEC_TYPE(float,     vec,    GLSL_TYPE_FLOAT,   GL_FLOAT)
DECL_VEC_TYPE(float16_t, f16vec, GLSL_TYPE_FLOAT16, GL_FLOAT16, _NV)
DECL_VEC_TYPE(double,    dvec,   GLSL_TYPE_DOUBLE,  GL_DOUBLE)
DECL_VEC_TYPE(int64_t,   i64vec, GLSL_TYPE_INT64,   GL_INT64, _ARB)
DECL_VEC_TYPE(uint64_t,  u64vec, GLSL_TYPE_UINT64,  GL_UNSIGNED_INT64, _ARB)
DECL_VEC_TYPE(int16_t,   i16vec, GLSL_TYPE_INT16,   GL_INT16, _NV)
DECL_VEC_TYPE(uint16_t,  u16vec, GLSL_TYPE_UINT16,  GL_UNSIGNED_INT16, _NV)
DECL_VEC_TYPE(int8_t,    i8vec,  GLSL_TYPE_INT8,    GL_INT8, _NV)
DECL_VEC_TYPE(uint8_t,   u8vec,  GLSL_TYPE_UINT8,   GL_UNSIGNED_INT8, _NV)

DECL_TYPE(mat2,   GL_FLOAT_MAT2,   GLSL_TYPE_FLOAT, 2, 2)
DECL_TYPE(mat3,   GL_FLOAT_MAT3,   GLSL_TYPE_FLOAT, 3, 3)
DECL_TYPE(mat4,   GL_FLOAT_MAT4,   GLSL_TYPE_FLOAT, 4, 4)

DECL_TYPE(mat2x3, GL_FLOAT_MAT2x3, GLSL_TYPE_FLOAT, 3, 2)
DECL_TYPE(mat2x4, GL_FLOAT_MAT2x4, GLSL_TYPE_FLOAT, 4, 2)
DECL_TYPE(mat3x2, GL_FLOAT_MAT3x2, GLSL_TYPE_FLOAT, 2, 3)
DECL_TYPE(mat3x4, GL_FLOAT_MAT3x4, GLSL_TYPE_FLOAT, 4, 3)
DECL_TYPE(mat4x2, GL_FLOAT_MAT4x2, GLSL_TYPE_FLOAT, 2, 4)
DECL_TYPE(mat4x3, GL_FLOAT_MAT4x3, GLSL_TYPE_FLOAT, 3, 4)

https://programtalk.com/vs2/?source=python/8661/BlenderPanda/OpenGL/raw/GL/ARB/vertex_shader.py

https://developer.android.com/develop/ui/views/graphics/opengl/draw

!! https://learnopengl.com/Getting-started/Hello-Triangle

Colors are RGBA and also stored in a vec4 ?! I don't think so.


1. creating memory on the GPU for the vertex data. <- vertex buffer objects (VBO)
2. configure how OpenGL should interpret the memory
3. specify how to send the data to the graphics card.

https://learnopengl.com/Getting-Started/OpenGL OpenGL objects

OpenGL's core-profile: minimal.

OpenGL is by itself a large state machine: a collection of variables that define how OpenGL should currently operate. The state of OpenGL is commonly referred to as the OpenGL context. When using OpenGL, we often change its state by setting some options, manipulating some buffers and then render using the current context.

# Objects

// create object
unsigned int objectId = 0;
glGenObject(1, &objectId);
// bind/assign object to context
glBindObject(GL_WINDOW_TARGET, objectId);
// set options of object currently bound to GL_WINDOW_TARGET
glSetObjectOption(GL_WINDOW_TARGET, GL_OPTION_WINDOW_WIDTH,  800);
glSetObjectOption(GL_WINDOW_TARGET, GL_OPTION_WINDOW_HEIGHT, 600);
// set context target back to default
glBindObject(GL_WINDOW_TARGET, 0);

# vertex buffer objects (VBO)

unsigned int VBO;
glGenBuffers(1, &VBO);
// buffer type of a vertex buffer object is GL_ARRAY_BUFFER. OpenGL allows us to bind to several buffers at once as long as they have a different buffer type.
glBindBuffer(GL_ARRAY_BUFFER, VBO);

float vertices[] = {
    -0.5f, -0.5f, 0.0f,
     0.5f, -0.5f, 0.0f,
     0.0f,  0.5f, 0.0f
};

glBufferData is a function specifically targeted to copy user-defined data into the currently bound buffer. Its first argument is the type of the buffer we want to copy data into: the vertex buffer object currently bound to the GL_ARRAY_BUFFER target. The second argument specifies the size of the data (in bytes) we want to pass to the buffer; a simple sizeof of the vertex data suffices. The third parameter is the actual data we want to send.

The fourth parameter specifies how we want the graphics card to manage the given data. This can take 3 forms:

GL_STREAM_DRAW: the data is set only once and used by the GPU at most a few times.
GL_STATIC_DRAW: the data is set only once and used many times.
GL_DYNAMIC_DRAW: the data is changed a lot and used many times.
The position data of the triangle does not change, is used a lot, and stays the same for every render call so its usage type should best be GL_STATIC_DRAW. If, for instance, one would have a buffer with data that is likely to change frequently, a usage type of GL_DYNAMIC_DRAW ensures the graphics card will place the data in memory that allows for faster writes.

//copies the previously defined vertex data into the buffer's memory:
glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

# GLSL vertex shader

#version 330 core
layout (location = 0) in vec3 aPos;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}

const char *vertexShaderSource = "#version 330 core\n"
    "layout (location = 0) in vec3 aPos;\n"
    "void main()\n"
    "{\n"
    "   gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);\n"
    "}\0";

unsigned int vertexShader;
vertexShader = glCreateShader(GL_VERTEX_SHADER);
glShaderSource(vertexShader, 1, &vertexShaderSource, NULL);
glCompileShader(vertexShader);

int  success;
char infoLog[512];
glGetShaderiv(vertexShader, GL_COMPILE_STATUS, &success);

if(!success)
{
    glGetShaderInfoLog(vertexShader, 512, NULL, infoLog);
    std::cout << "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n" << infoLog << std::endl;
}

# Fragment shader

#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}

unsigned int fragmentShader;
fragmentShader = glCreateShader(GL_FRAGMENT_SHADER);
glShaderSource(fragmentShader, 1, &fragmentShaderSource, NULL);
glCompileShader(fragmentShader);

^ https://learnopengl.com/Getting-started/Hello-Triangle

*/

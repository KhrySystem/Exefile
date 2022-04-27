pub use self::types::*;
pub use self::enumerations::*;
pub use self::functions::*;

use std::os::raw::c_void;


#[derive(Copy, Clone)]
struct FnPtr {
    ptr: *const c_void,
    is_loaded: bool
}

#[allow(dead_code)]
impl FnPtr {
    fn new(ptr: *const c_void) -> FnPtr {
        if !ptr.is_null() {
            FnPtr { ptr, is_loaded: true }
        } else {
            FnPtr { ptr: FnPtr::not_initialized as *const c_void, is_loaded: false }
        }
    }

    fn set_ptr(&mut self, ptr: *const c_void) {
        *self = Self::new(ptr);
    }
    
    fn aliased(&mut self, other: &FnPtr) {
        if !self.is_loaded && other.is_loaded {
            *self = *other;
        }
    }

    #[inline(never)]
    fn not_initialized() -> ! { panic!("wgl: function not initialized") }
}

unsafe impl Sync for FnPtr {}
unsafe impl Send for FnPtr {}

pub mod types {
#![allow(dead_code, non_camel_case_types, non_snake_case)]
#![allow(dead_code, non_snake_case, non_camel_case_types)]

use std::os::raw;

pub type GLvoid = raw::c_void;

pub type GLbyte = raw::c_char;
pub type GLubyte = raw::c_uchar;
pub type GLchar = raw::c_char;
pub type GLboolean = raw::c_uchar;

pub type GLshort = raw::c_short;
pub type GLushort = raw::c_ushort;

pub type GLint = raw::c_int;
pub type GLuint = raw::c_uint;
pub type GLint64 = i64;
pub type GLuint64 = u64;

pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;

pub type GLsizei = GLint;
pub type GLclampx = raw::c_int;
pub type GLfixed = GLint;
pub type GLhalf = raw::c_ushort;
pub type GLhalfNV = raw::c_ushort;
pub type GLhalfARB = raw::c_ushort;

pub type GLenum = raw::c_uint;
pub type GLbitfield = raw::c_uint;

pub type GLfloat = raw::c_float;
pub type GLdouble = raw::c_double;
pub type GLclampf = raw::c_float;
pub type GLclampd = raw::c_double;

pub type GLcharARB = raw::c_char;

#[cfg(target_os = "macos")]
pub type GLhandleARB = *const raw::c_void;
#[cfg(not(target_os = "macos"))]
pub type GLhandleARB = raw::c_uint;

pub enum __GLsync {}

pub type GLsync = *const __GLsync;

pub enum _cl_context {}

pub enum _cl_event {}

pub type GLvdpauSurfaceNV = GLintptr;
pub type GLeglClientBufferEXT = *const raw::c_void;
pub type GLeglImageOES = *const raw::c_void;


pub type GLDEBUGPROC = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut raw::c_void,
);
pub type GLDEBUGPROCARB = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut raw::c_void,
);
pub type GLDEBUGPROCKHR = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut GLvoid,
);
pub type GLDEBUGPROCAMD = extern "system" fn (
    id: GLuint,
    category: GLenum,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut GLvoid,
);
pub type GLVULKANPROCNV = extern "system" fn ();

use std;

pub type BOOL = std::os::raw::c_int;
pub type BYTE = std::os::raw::c_uchar;
pub type CHAR = std::os::raw::c_char;
pub type COLORREF = DWORD;
pub type DWORD = std::os::raw::c_ulong;
pub type FLOAT = std::os::raw::c_float;
pub type HANDLE = PVOID;
pub type HDC = HANDLE;
pub type HENHMETAFILE = HANDLE;
pub type HGLRC = *const std::os::raw::c_void;
pub type HGPUNV = *const std::os::raw::c_void;
pub type HPBUFFERARB = *const std::os::raw::c_void;
pub type HPBUFFEREXT = *const std::os::raw::c_void;
pub type HPGPUNV = *const std::os::raw::c_void;
pub type HPVIDEODEV = *const std::os::raw::c_void;
pub type HVIDEOINPUTDEVICENV = *const std::os::raw::c_void;
pub type HVIDEOOUTPUTDEVICENV = *const std::os::raw::c_void;
pub type INT = std::os::raw::c_int;
pub type INT32 = i32;
pub type INT64 = i64;
pub type LONG = std::os::raw::c_long;
pub type LPCSTR = *const std::os::raw::c_char;
pub type LPVOID = *const std::os::raw::c_void;
pub type PVOID = *const std::os::raw::c_void;
pub type UINT = std::os::raw::c_uint;
pub type USHORT = std::os::raw::c_ushort;
pub type VOID = ();
pub type WORD = std::os::raw::c_ushort;

pub enum __PROC_fn {}
pub type PROC = *mut __PROC_fn;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RECT {
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct POINTFLOAT {
    pub x: FLOAT,
    pub y: FLOAT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GLYPHMETRICSFLOAT {
    pub gmfBlackBoxX: FLOAT,
    pub gmfBlackBoxY: FLOAT,
    pub gmfptGlyphOrigin: POINTFLOAT,
    pub gmfCellIncX: FLOAT,
    pub gmfCellIncY: FLOAT,
}
pub type LPGLYPHMETRICSFLOAT = *const GLYPHMETRICSFLOAT;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LAYERPLANEDESCRIPTOR {
    pub nSize: WORD,
    pub nVersion: WORD,
    pub dwFlags: DWORD,
    pub iPixelType: BYTE,
    pub cColorBits: BYTE,
    pub cRedBits: BYTE,
    pub cRedShift: BYTE,
    pub cGreenBits: BYTE,
    pub cGreenShift: BYTE,
    pub cBlueBits: BYTE,
    pub cBlueShift: BYTE,
    pub cAlphaBits: BYTE,
    pub cAlphaShift: BYTE,
    pub cAccumBits: BYTE,
    pub cAccumRedBits: BYTE,
    pub cAccumGreenBits: BYTE,
    pub cAccumBlueBits: BYTE,
    pub cAccumAlphaBits: BYTE,
    pub cDepthBits: BYTE,
    pub cStencilBits: BYTE,
    pub cAuxBuffers: BYTE,
    pub iLayerType: BYTE,
    pub bReserved: BYTE,
    pub crTransparent: COLORREF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PIXELFORMATDESCRIPTOR {
    pub nSize: WORD,
    pub nVersion: WORD,
    pub dwFlags: DWORD,
    pub iPixelType: BYTE,
    pub cColorBits: BYTE,
    pub cRedBits: BYTE,
    pub cRedShift: BYTE,
    pub cGreenBits: BYTE,
    pub cGreenShift: BYTE,
    pub cBlueBits: BYTE,
    pub cBlueShift: BYTE,
    pub cAlphaBits: BYTE,
    pub cAlphaShift: BYTE,
    pub cAccumBits: BYTE,
    pub cAccumRedBits: BYTE,
    pub cAccumGreenBits: BYTE,
    pub cAccumBlueBits: BYTE,
    pub cAccumAlphaBits: BYTE,
    pub cDepthBits: BYTE,
    pub cStencilBits: BYTE,
    pub cAuxBuffers: BYTE,
    pub iLayerType: BYTE,
    pub bReserved: BYTE,
    pub dwLayerMask: DWORD,
    pub dwVisibleMask: DWORD,
    pub dwDamageMask: DWORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _GPU_DEVICE {
    cb: DWORD,
    DeviceName: [CHAR; 32],
    DeviceString: [CHAR; 128],
    Flags: DWORD,
    rcVirtualScreen: RECT,
}

pub struct GPU_DEVICE(_GPU_DEVICE);
pub struct PGPU_DEVICE(*const _GPU_DEVICE);
}

pub mod enumerations {
    #![allow(dead_code, non_upper_case_globals, unused_imports)]

    use std::os::raw::*;
    use super::types::*;

    pub const ERROR_INCOMPATIBLE_AFFINITY_MASKS_NV: c_uint = 0x20D0;
    pub const ERROR_INCOMPATIBLE_DEVICE_CONTEXTS_ARB: c_uint = 0x2054;
    pub const ERROR_INVALID_PIXEL_TYPE_ARB: c_uint = 0x2043;
    pub const ERROR_INVALID_PIXEL_TYPE_EXT: c_uint = 0x2043;
    pub const ERROR_INVALID_PROFILE_ARB: c_uint = 0x2096;
    pub const ERROR_INVALID_VERSION_ARB: c_uint = 0x2095;
    pub const ERROR_MISSING_AFFINITY_MASK_NV: c_uint = 0x20D1;
    pub const ACCELERATION_ARB: c_uint = 0x2003;
    pub const ACCELERATION_EXT: c_uint = 0x2003;
    pub const ACCESS_READ_ONLY_NV: c_uint = 0x00000000;
    pub const ACCESS_READ_WRITE_NV: c_uint = 0x00000001;
    pub const ACCESS_WRITE_DISCARD_NV: c_uint = 0x00000002;
    pub const ACCUM_ALPHA_BITS_ARB: c_uint = 0x2021;
    pub const ACCUM_ALPHA_BITS_EXT: c_uint = 0x2021;
    pub const ACCUM_BITS_ARB: c_uint = 0x201D;
    pub const ACCUM_BITS_EXT: c_uint = 0x201D;
    pub const ACCUM_BLUE_BITS_ARB: c_uint = 0x2020;
    pub const ACCUM_BLUE_BITS_EXT: c_uint = 0x2020;
    pub const ACCUM_GREEN_BITS_ARB: c_uint = 0x201F;
    pub const ACCUM_GREEN_BITS_EXT: c_uint = 0x201F;
    pub const ACCUM_RED_BITS_ARB: c_uint = 0x201E;
    pub const ACCUM_RED_BITS_EXT: c_uint = 0x201E;
    pub const ALPHA_BITS_ARB: c_uint = 0x201B;
    pub const ALPHA_BITS_EXT: c_uint = 0x201B;
    pub const ALPHA_SHIFT_ARB: c_uint = 0x201C;
    pub const ALPHA_SHIFT_EXT: c_uint = 0x201C;
    pub const AUX0_ARB: c_uint = 0x2087;
    pub const AUX1_ARB: c_uint = 0x2088;
    pub const AUX2_ARB: c_uint = 0x2089;
    pub const AUX3_ARB: c_uint = 0x208A;
    pub const AUX4_ARB: c_uint = 0x208B;
    pub const AUX5_ARB: c_uint = 0x208C;
    pub const AUX6_ARB: c_uint = 0x208D;
    pub const AUX7_ARB: c_uint = 0x208E;
    pub const AUX8_ARB: c_uint = 0x208F;
    pub const AUX9_ARB: c_uint = 0x2090;
    pub const AUX_BUFFERS_ARB: c_uint = 0x2024;
    pub const AUX_BUFFERS_EXT: c_uint = 0x2024;
    pub const BACK_COLOR_BUFFER_BIT_ARB: c_uint = 0x00000002;
    pub const BACK_LEFT_ARB: c_uint = 0x2085;
    pub const BACK_RIGHT_ARB: c_uint = 0x2086;
    pub const BIND_TO_TEXTURE_DEPTH_NV: c_uint = 0x20A3;
    pub const BIND_TO_TEXTURE_RECTANGLE_DEPTH_NV: c_uint = 0x20A4;
    pub const BIND_TO_TEXTURE_RECTANGLE_FLOAT_RGBA_NV: c_uint = 0x20B4;
    pub const BIND_TO_TEXTURE_RECTANGLE_FLOAT_RGB_NV: c_uint = 0x20B3;
    pub const BIND_TO_TEXTURE_RECTANGLE_FLOAT_RG_NV: c_uint = 0x20B2;
    pub const BIND_TO_TEXTURE_RECTANGLE_FLOAT_R_NV: c_uint = 0x20B1;
    pub const BIND_TO_TEXTURE_RECTANGLE_RGBA_NV: c_uint = 0x20A1;
    pub const BIND_TO_TEXTURE_RECTANGLE_RGB_NV: c_uint = 0x20A0;
    pub const BIND_TO_TEXTURE_RGBA_ARB: c_uint = 0x2071;
    pub const BIND_TO_TEXTURE_RGB_ARB: c_uint = 0x2070;
    pub const BIND_TO_VIDEO_RGBA_NV: c_uint = 0x20C1;
    pub const BIND_TO_VIDEO_RGB_AND_DEPTH_NV: c_uint = 0x20C2;
    pub const BIND_TO_VIDEO_RGB_NV: c_uint = 0x20C0;
    pub const BLUE_BITS_ARB: c_uint = 0x2019;
    pub const BLUE_BITS_EXT: c_uint = 0x2019;
    pub const BLUE_SHIFT_ARB: c_uint = 0x201A;
    pub const BLUE_SHIFT_EXT: c_uint = 0x201A;
    pub const COLORSPACE_EXT: c_uint = 0x309D;
    pub const COLORSPACE_LINEAR_EXT: c_uint = 0x308A;
    pub const COLORSPACE_SRGB_EXT: c_uint = 0x3089;
    pub const COLOR_BITS_ARB: c_uint = 0x2014;
    pub const COLOR_BITS_EXT: c_uint = 0x2014;
    pub const COLOR_SAMPLES_NV: c_uint = 0x20B9;
    pub const CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: c_uint = 0x00000002;
    pub const CONTEXT_CORE_PROFILE_BIT_ARB: c_uint = 0x00000001;
    pub const CONTEXT_DEBUG_BIT_ARB: c_uint = 0x00000001;
    pub const CONTEXT_ES2_PROFILE_BIT_EXT: c_uint = 0x00000004;
    pub const CONTEXT_ES_PROFILE_BIT_EXT: c_uint = 0x00000004;
    pub const CONTEXT_FLAGS_ARB: c_uint = 0x2094;
    pub const CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: c_uint = 0x00000002;
    pub const CONTEXT_LAYER_PLANE_ARB: c_uint = 0x2093;
    pub const CONTEXT_MAJOR_VERSION_ARB: c_uint = 0x2091;
    pub const CONTEXT_MINOR_VERSION_ARB: c_uint = 0x2092;
    pub const CONTEXT_MULTIGPU_ATTRIB_AFR_NV: c_uint = 0x20AC;
    pub const CONTEXT_MULTIGPU_ATTRIB_MULTICAST_NV: c_uint = 0x20AD;
    pub const CONTEXT_MULTIGPU_ATTRIB_MULTI_DISPLAY_MULTICAST_NV: c_uint = 0x20AE;
    pub const CONTEXT_MULTIGPU_ATTRIB_NV: c_uint = 0x20AA;
    pub const CONTEXT_MULTIGPU_ATTRIB_SINGLE_NV: c_uint = 0x20AB;
    pub const CONTEXT_OPENGL_NO_ERROR_ARB: c_uint = 0x31B3;
    pub const CONTEXT_PROFILE_MASK_ARB: c_uint = 0x9126;
    pub const CONTEXT_RELEASE_BEHAVIOR_ARB: c_uint = 0x2097;
    pub const CONTEXT_RELEASE_BEHAVIOR_FLUSH_ARB: c_uint = 0x2098;
    pub const CONTEXT_RELEASE_BEHAVIOR_NONE_ARB: c_uint = 0;
    pub const CONTEXT_RESET_ISOLATION_BIT_ARB: c_uint = 0x00000008;
    pub const CONTEXT_RESET_NOTIFICATION_STRATEGY_ARB: c_uint = 0x8256;
    pub const CONTEXT_ROBUST_ACCESS_BIT_ARB: c_uint = 0x00000004;
    pub const COVERAGE_SAMPLES_NV: c_uint = 0x2042;
    pub const CUBE_MAP_FACE_ARB: c_uint = 0x207C;
    pub const DEPTH_BITS_ARB: c_uint = 0x2022;
    pub const DEPTH_BITS_EXT: c_uint = 0x2022;
    pub const DEPTH_BUFFER_BIT_ARB: c_uint = 0x00000004;
    pub const DEPTH_COMPONENT_NV: c_uint = 0x20A7;
    pub const DEPTH_FLOAT_EXT: c_uint = 0x2040;
    pub const DEPTH_TEXTURE_FORMAT_NV: c_uint = 0x20A5;
    pub const DIGITAL_VIDEO_CURSOR_ALPHA_FRAMEBUFFER_I3D: c_uint = 0x2050;
    pub const DIGITAL_VIDEO_CURSOR_ALPHA_VALUE_I3D: c_uint = 0x2051;
    pub const DIGITAL_VIDEO_CURSOR_INCLUDED_I3D: c_uint = 0x2052;
    pub const DIGITAL_VIDEO_GAMMA_CORRECTED_I3D: c_uint = 0x2053;
    pub const DOUBLE_BUFFER_ARB: c_uint = 0x2011;
    pub const DOUBLE_BUFFER_EXT: c_uint = 0x2011;
    pub const DRAW_TO_BITMAP_ARB: c_uint = 0x2002;
    pub const DRAW_TO_BITMAP_EXT: c_uint = 0x2002;
    pub const DRAW_TO_PBUFFER_ARB: c_uint = 0x202D;
    pub const DRAW_TO_PBUFFER_EXT: c_uint = 0x202D;
    pub const DRAW_TO_WINDOW_ARB: c_uint = 0x2001;
    pub const DRAW_TO_WINDOW_EXT: c_uint = 0x2001;
    pub const FLOAT_COMPONENTS_NV: c_uint = 0x20B0;
    pub const FONT_LINES: c_uint = 0;
    pub const FONT_POLYGONS: c_uint = 1;
    pub const FRAMEBUFFER_SRGB_CAPABLE_ARB: c_uint = 0x20A9;
    pub const FRAMEBUFFER_SRGB_CAPABLE_EXT: c_uint = 0x20A9;
    pub const FRONT_COLOR_BUFFER_BIT_ARB: c_uint = 0x00000001;
    pub const FRONT_LEFT_ARB: c_uint = 0x2083;
    pub const FRONT_RIGHT_ARB: c_uint = 0x2084;
    pub const FULL_ACCELERATION_ARB: c_uint = 0x2027;
    pub const FULL_ACCELERATION_EXT: c_uint = 0x2027;
    pub const GAMMA_EXCLUDE_DESKTOP_I3D: c_uint = 0x204F;
    pub const GAMMA_TABLE_SIZE_I3D: c_uint = 0x204E;
    pub const GENERIC_ACCELERATION_ARB: c_uint = 0x2026;
    pub const GENERIC_ACCELERATION_EXT: c_uint = 0x2026;
    pub const GENLOCK_SOURCE_DIGITAL_FIELD_I3D: c_uint = 0x2049;
    pub const GENLOCK_SOURCE_DIGITAL_SYNC_I3D: c_uint = 0x2048;
    pub const GENLOCK_SOURCE_EDGE_BOTH_I3D: c_uint = 0x204C;
    pub const GENLOCK_SOURCE_EDGE_FALLING_I3D: c_uint = 0x204A;
    pub const GENLOCK_SOURCE_EDGE_RISING_I3D: c_uint = 0x204B;
    pub const GENLOCK_SOURCE_EXTERNAL_FIELD_I3D: c_uint = 0x2046;
    pub const GENLOCK_SOURCE_EXTERNAL_SYNC_I3D: c_uint = 0x2045;
    pub const GENLOCK_SOURCE_EXTERNAL_TTL_I3D: c_uint = 0x2047;
    pub const GENLOCK_SOURCE_MULTIVIEW_I3D: c_uint = 0x2044;
    pub const GPU_CLOCK_AMD: c_uint = 0x21A4;
    pub const GPU_FASTEST_TARGET_GPUS_AMD: c_uint = 0x21A2;
    pub const GPU_NUM_PIPES_AMD: c_uint = 0x21A5;
    pub const GPU_NUM_RB_AMD: c_uint = 0x21A7;
    pub const GPU_NUM_SIMD_AMD: c_uint = 0x21A6;
    pub const GPU_NUM_SPI_AMD: c_uint = 0x21A8;
    pub const GPU_OPENGL_VERSION_STRING_AMD: c_uint = 0x1F02;
    pub const GPU_RAM_AMD: c_uint = 0x21A3;
    pub const GPU_RENDERER_STRING_AMD: c_uint = 0x1F01;
    pub const GPU_VENDOR_AMD: c_uint = 0x1F00;
    pub const GREEN_BITS_ARB: c_uint = 0x2017;
    pub const GREEN_BITS_EXT: c_uint = 0x2017;
    pub const GREEN_SHIFT_ARB: c_uint = 0x2018;
    pub const GREEN_SHIFT_EXT: c_uint = 0x2018;
    pub const IMAGE_BUFFER_LOCK_I3D: c_uint = 0x00000002;
    pub const IMAGE_BUFFER_MIN_ACCESS_I3D: c_uint = 0x00000001;
    pub const LOSE_CONTEXT_ON_RESET_ARB: c_uint = 0x8252;
    pub const MAX_PBUFFER_HEIGHT_ARB: c_uint = 0x2030;
    pub const MAX_PBUFFER_HEIGHT_EXT: c_uint = 0x2030;
    pub const MAX_PBUFFER_PIXELS_ARB: c_uint = 0x202E;
    pub const MAX_PBUFFER_PIXELS_EXT: c_uint = 0x202E;
    pub const MAX_PBUFFER_WIDTH_ARB: c_uint = 0x202F;
    pub const MAX_PBUFFER_WIDTH_EXT: c_uint = 0x202F;
    pub const MIPMAP_LEVEL_ARB: c_uint = 0x207B;
    pub const MIPMAP_TEXTURE_ARB: c_uint = 0x2074;
    pub const NEED_PALETTE_ARB: c_uint = 0x2004;
    pub const NEED_PALETTE_EXT: c_uint = 0x2004;
    pub const NEED_SYSTEM_PALETTE_ARB: c_uint = 0x2005;
    pub const NEED_SYSTEM_PALETTE_EXT: c_uint = 0x2005;
    pub const NO_ACCELERATION_ARB: c_uint = 0x2025;
    pub const NO_ACCELERATION_EXT: c_uint = 0x2025;
    pub const NO_RESET_NOTIFICATION_ARB: c_uint = 0x8261;
    pub const NO_TEXTURE_ARB: c_uint = 0x2077;
    pub const NUMBER_OVERLAYS_ARB: c_uint = 0x2008;
    pub const NUMBER_OVERLAYS_EXT: c_uint = 0x2008;
    pub const NUMBER_PIXEL_FORMATS_ARB: c_uint = 0x2000;
    pub const NUMBER_PIXEL_FORMATS_EXT: c_uint = 0x2000;
    pub const NUMBER_UNDERLAYS_ARB: c_uint = 0x2009;
    pub const NUMBER_UNDERLAYS_EXT: c_uint = 0x2009;
    pub const NUM_VIDEO_CAPTURE_SLOTS_NV: c_uint = 0x20CF;
    pub const NUM_VIDEO_SLOTS_NV: c_uint = 0x20F0;
    pub const OPTIMAL_PBUFFER_HEIGHT_EXT: c_uint = 0x2032;
    pub const OPTIMAL_PBUFFER_WIDTH_EXT: c_uint = 0x2031;
    pub const PBUFFER_HEIGHT_ARB: c_uint = 0x2035;
    pub const PBUFFER_HEIGHT_EXT: c_uint = 0x2035;
    pub const PBUFFER_LARGEST_ARB: c_uint = 0x2033;
    pub const PBUFFER_LARGEST_EXT: c_uint = 0x2033;
    pub const PBUFFER_LOST_ARB: c_uint = 0x2036;
    pub const PBUFFER_WIDTH_ARB: c_uint = 0x2034;
    pub const PBUFFER_WIDTH_EXT: c_uint = 0x2034;
    pub const PIXEL_TYPE_ARB: c_uint = 0x2013;
    pub const PIXEL_TYPE_EXT: c_uint = 0x2013;
    pub const RED_BITS_ARB: c_uint = 0x2015;
    pub const RED_BITS_EXT: c_uint = 0x2015;
    pub const RED_SHIFT_ARB: c_uint = 0x2016;
    pub const RED_SHIFT_EXT: c_uint = 0x2016;
    pub const SAMPLES_3DFX: c_uint = 0x2061;
    pub const SAMPLES_ARB: c_uint = 0x2042;
    pub const SAMPLES_EXT: c_uint = 0x2042;
    pub const SAMPLE_BUFFERS_3DFX: c_uint = 0x2060;
    pub const SAMPLE_BUFFERS_ARB: c_uint = 0x2041;
    pub const SAMPLE_BUFFERS_EXT: c_uint = 0x2041;
    pub const SHARE_ACCUM_ARB: c_uint = 0x200E;
    pub const SHARE_ACCUM_EXT: c_uint = 0x200E;
    pub const SHARE_DEPTH_ARB: c_uint = 0x200C;
    pub const SHARE_DEPTH_EXT: c_uint = 0x200C;
    pub const SHARE_STENCIL_ARB: c_uint = 0x200D;
    pub const SHARE_STENCIL_EXT: c_uint = 0x200D;
    pub const STENCIL_BITS_ARB: c_uint = 0x2023;
    pub const STENCIL_BITS_EXT: c_uint = 0x2023;
    pub const STENCIL_BUFFER_BIT_ARB: c_uint = 0x00000008;
    pub const STEREO_ARB: c_uint = 0x2012;
    pub const STEREO_EMITTER_DISABLE_3DL: c_uint = 0x2056;
    pub const STEREO_EMITTER_ENABLE_3DL: c_uint = 0x2055;
    pub const STEREO_EXT: c_uint = 0x2012;
    pub const STEREO_POLARITY_INVERT_3DL: c_uint = 0x2058;
    pub const STEREO_POLARITY_NORMAL_3DL: c_uint = 0x2057;
    pub const SUPPORT_GDI_ARB: c_uint = 0x200F;
    pub const SUPPORT_GDI_EXT: c_uint = 0x200F;
    pub const SUPPORT_OPENGL_ARB: c_uint = 0x2010;
    pub const SUPPORT_OPENGL_EXT: c_uint = 0x2010;
    pub const SWAP_COPY_ARB: c_uint = 0x2029;
    pub const SWAP_COPY_EXT: c_uint = 0x2029;
    pub const SWAP_EXCHANGE_ARB: c_uint = 0x2028;
    pub const SWAP_EXCHANGE_EXT: c_uint = 0x2028;
    pub const SWAP_LAYER_BUFFERS_ARB: c_uint = 0x2006;
    pub const SWAP_LAYER_BUFFERS_EXT: c_uint = 0x2006;
    pub const SWAP_MAIN_PLANE: c_uint = 0x00000001;
    pub const SWAP_METHOD_ARB: c_uint = 0x2007;
    pub const SWAP_METHOD_EXT: c_uint = 0x2007;
    pub const SWAP_OVERLAY1: c_uint = 0x00000002;
    pub const SWAP_OVERLAY10: c_uint = 0x00000400;
    pub const SWAP_OVERLAY11: c_uint = 0x00000800;
    pub const SWAP_OVERLAY12: c_uint = 0x00001000;
    pub const SWAP_OVERLAY13: c_uint = 0x00002000;
    pub const SWAP_OVERLAY14: c_uint = 0x00004000;
    pub const SWAP_OVERLAY15: c_uint = 0x00008000;
    pub const SWAP_OVERLAY2: c_uint = 0x00000004;
    pub const SWAP_OVERLAY3: c_uint = 0x00000008;
    pub const SWAP_OVERLAY4: c_uint = 0x00000010;
    pub const SWAP_OVERLAY5: c_uint = 0x00000020;
    pub const SWAP_OVERLAY6: c_uint = 0x00000040;
    pub const SWAP_OVERLAY7: c_uint = 0x00000080;
    pub const SWAP_OVERLAY8: c_uint = 0x00000100;
    pub const SWAP_OVERLAY9: c_uint = 0x00000200;
    pub const SWAP_UNDEFINED_ARB: c_uint = 0x202A;
    pub const SWAP_UNDEFINED_EXT: c_uint = 0x202A;
    pub const SWAP_UNDERLAY1: c_uint = 0x00010000;
    pub const SWAP_UNDERLAY10: c_uint = 0x02000000;
    pub const SWAP_UNDERLAY11: c_uint = 0x04000000;
    pub const SWAP_UNDERLAY12: c_uint = 0x08000000;
    pub const SWAP_UNDERLAY13: c_uint = 0x10000000;
    pub const SWAP_UNDERLAY14: c_uint = 0x20000000;
    pub const SWAP_UNDERLAY15: c_uint = 0x40000000;
    pub const SWAP_UNDERLAY2: c_uint = 0x00020000;
    pub const SWAP_UNDERLAY3: c_uint = 0x00040000;
    pub const SWAP_UNDERLAY4: c_uint = 0x00080000;
    pub const SWAP_UNDERLAY5: c_uint = 0x00100000;
    pub const SWAP_UNDERLAY6: c_uint = 0x00200000;
    pub const SWAP_UNDERLAY7: c_uint = 0x00400000;
    pub const SWAP_UNDERLAY8: c_uint = 0x00800000;
    pub const SWAP_UNDERLAY9: c_uint = 0x01000000;
    pub const TEXTURE_1D_ARB: c_uint = 0x2079;
    pub const TEXTURE_2D_ARB: c_uint = 0x207A;
    pub const TEXTURE_CUBE_MAP_ARB: c_uint = 0x2078;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_X_ARB: c_uint = 0x207E;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y_ARB: c_uint = 0x2080;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z_ARB: c_uint = 0x2082;
    pub const TEXTURE_CUBE_MAP_POSITIVE_X_ARB: c_uint = 0x207D;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Y_ARB: c_uint = 0x207F;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Z_ARB: c_uint = 0x2081;
    pub const TEXTURE_DEPTH_COMPONENT_NV: c_uint = 0x20A6;
    pub const TEXTURE_FLOAT_RGBA_NV: c_uint = 0x20B8;
    pub const TEXTURE_FLOAT_RGB_NV: c_uint = 0x20B7;
    pub const TEXTURE_FLOAT_RG_NV: c_uint = 0x20B6;
    pub const TEXTURE_FLOAT_R_NV: c_uint = 0x20B5;
    pub const TEXTURE_FORMAT_ARB: c_uint = 0x2072;
    pub const TEXTURE_RECTANGLE_ATI: c_uint = 0x21A5;
    pub const TEXTURE_RECTANGLE_NV: c_uint = 0x20A2;
    pub const TEXTURE_RGBA_ARB: c_uint = 0x2076;
    pub const TEXTURE_RGB_ARB: c_uint = 0x2075;
    pub const TEXTURE_TARGET_ARB: c_uint = 0x2073;
    pub const TRANSPARENT_ALPHA_VALUE_ARB: c_uint = 0x203A;
    pub const TRANSPARENT_ARB: c_uint = 0x200A;
    pub const TRANSPARENT_BLUE_VALUE_ARB: c_uint = 0x2039;
    pub const TRANSPARENT_EXT: c_uint = 0x200A;
    pub const TRANSPARENT_GREEN_VALUE_ARB: c_uint = 0x2038;
    pub const TRANSPARENT_INDEX_VALUE_ARB: c_uint = 0x203B;
    pub const TRANSPARENT_RED_VALUE_ARB: c_uint = 0x2037;
    pub const TRANSPARENT_VALUE_EXT: c_uint = 0x200B;
    pub const TYPE_COLORINDEX_ARB: c_uint = 0x202C;
    pub const TYPE_COLORINDEX_EXT: c_uint = 0x202C;
    pub const TYPE_RGBA_ARB: c_uint = 0x202B;
    pub const TYPE_RGBA_EXT: c_uint = 0x202B;
    pub const TYPE_RGBA_FLOAT_ARB: c_uint = 0x21A0;
    pub const TYPE_RGBA_FLOAT_ATI: c_uint = 0x21A0;
    pub const TYPE_RGBA_UNSIGNED_FLOAT_EXT: c_uint = 0x20A8;
    pub const UNIQUE_ID_NV: c_uint = 0x20CE;
    pub const VIDEO_OUT_ALPHA_NV: c_uint = 0x20C4;
    pub const VIDEO_OUT_COLOR_AND_ALPHA_NV: c_uint = 0x20C6;
    pub const VIDEO_OUT_COLOR_AND_DEPTH_NV: c_uint = 0x20C7;
    pub const VIDEO_OUT_COLOR_NV: c_uint = 0x20C3;
    pub const VIDEO_OUT_DEPTH_NV: c_uint = 0x20C5;
    pub const VIDEO_OUT_FIELD_1: c_uint = 0x20C9;
    pub const VIDEO_OUT_FIELD_2: c_uint = 0x20CA;
    pub const VIDEO_OUT_FRAME: c_uint = 0x20C8;
    pub const VIDEO_OUT_STACKED_FIELDS_1_2: c_uint = 0x20CB;
    pub const VIDEO_OUT_STACKED_FIELDS_2_1: c_uint = 0x20CC;
}

pub mod functions {
    #![allow(non_snake_case, unused_variables, dead_code, unused_imports)]

    use std::mem::transmute;
    use std::os::raw::*;
    use super::*;
    use super::types::*;

    macro_rules! func {
        ($fun:ident, $ret:ty, $($name:ident: $typ:ty),*) => {
            #[inline] pub unsafe fn $fun(&self, $($name: $typ),*) -> $ret {
                transmute::<_, extern "system" fn($($typ),*) -> $ret>(self.$fun.ptr)($($name),*)
            }
        }
    }

    pub struct Wgl {
         pub(super) ChoosePixelFormat: FnPtr,
         pub(super) DescribePixelFormat: FnPtr,
         pub(super) GetEnhMetaFilePixelFormat: FnPtr,
         pub(super) GetPixelFormat: FnPtr,
         pub(super) SetPixelFormat: FnPtr,
         pub(super) SwapBuffers: FnPtr,
         pub(super) AllocateMemoryNV: FnPtr,
         pub(super) AssociateImageBufferEventsI3D: FnPtr,
         pub(super) BeginFrameTrackingI3D: FnPtr,
         pub(super) BindDisplayColorTableEXT: FnPtr,
         pub(super) BindSwapBarrierNV: FnPtr,
         pub(super) BindTexImageARB: FnPtr,
         pub(super) BindVideoCaptureDeviceNV: FnPtr,
         pub(super) BindVideoDeviceNV: FnPtr,
         pub(super) BindVideoImageNV: FnPtr,
         pub(super) BlitContextFramebufferAMD: FnPtr,
         pub(super) ChoosePixelFormatARB: FnPtr,
         pub(super) ChoosePixelFormatEXT: FnPtr,
         pub(super) CopyContext: FnPtr,
         pub(super) CopyImageSubDataNV: FnPtr,
         pub(super) CreateAffinityDCNV: FnPtr,
         pub(super) CreateAssociatedContextAMD: FnPtr,
         pub(super) CreateAssociatedContextAttribsAMD: FnPtr,
         pub(super) CreateBufferRegionARB: FnPtr,
         pub(super) CreateContext: FnPtr,
         pub(super) CreateContextAttribsARB: FnPtr,
         pub(super) CreateDisplayColorTableEXT: FnPtr,
         pub(super) CreateImageBufferI3D: FnPtr,
         pub(super) CreateLayerContext: FnPtr,
         pub(super) CreatePbufferARB: FnPtr,
         pub(super) CreatePbufferEXT: FnPtr,
         pub(super) DXCloseDeviceNV: FnPtr,
         pub(super) DXLockObjectsNV: FnPtr,
         pub(super) DXObjectAccessNV: FnPtr,
         pub(super) DXOpenDeviceNV: FnPtr,
         pub(super) DXRegisterObjectNV: FnPtr,
         pub(super) DXSetResourceShareHandleNV: FnPtr,
         pub(super) DXUnlockObjectsNV: FnPtr,
         pub(super) DXUnregisterObjectNV: FnPtr,
         pub(super) DelayBeforeSwapNV: FnPtr,
         pub(super) DeleteAssociatedContextAMD: FnPtr,
         pub(super) DeleteBufferRegionARB: FnPtr,
         pub(super) DeleteContext: FnPtr,
         pub(super) DeleteDCNV: FnPtr,
         pub(super) DescribeLayerPlane: FnPtr,
         pub(super) DestroyDisplayColorTableEXT: FnPtr,
         pub(super) DestroyImageBufferI3D: FnPtr,
         pub(super) DestroyPbufferARB: FnPtr,
         pub(super) DestroyPbufferEXT: FnPtr,
         pub(super) DisableFrameLockI3D: FnPtr,
         pub(super) DisableGenlockI3D: FnPtr,
         pub(super) EnableFrameLockI3D: FnPtr,
         pub(super) EnableGenlockI3D: FnPtr,
         pub(super) EndFrameTrackingI3D: FnPtr,
         pub(super) EnumGpuDevicesNV: FnPtr,
         pub(super) EnumGpusFromAffinityDCNV: FnPtr,
         pub(super) EnumGpusNV: FnPtr,
         pub(super) EnumerateVideoCaptureDevicesNV: FnPtr,
         pub(super) EnumerateVideoDevicesNV: FnPtr,
         pub(super) FreeMemoryNV: FnPtr,
         pub(super) GenlockSampleRateI3D: FnPtr,
         pub(super) GenlockSourceDelayI3D: FnPtr,
         pub(super) GenlockSourceEdgeI3D: FnPtr,
         pub(super) GenlockSourceI3D: FnPtr,
         pub(super) GetContextGPUIDAMD: FnPtr,
         pub(super) GetCurrentAssociatedContextAMD: FnPtr,
         pub(super) GetCurrentContext: FnPtr,
         pub(super) GetCurrentDC: FnPtr,
         pub(super) GetCurrentReadDCARB: FnPtr,
         pub(super) GetCurrentReadDCEXT: FnPtr,
         pub(super) GetDigitalVideoParametersI3D: FnPtr,
         pub(super) GetExtensionsStringARB: FnPtr,
         pub(super) GetExtensionsStringEXT: FnPtr,
         pub(super) GetFrameUsageI3D: FnPtr,
         pub(super) GetGPUIDsAMD: FnPtr,
         pub(super) GetGPUInfoAMD: FnPtr,
         pub(super) GetGammaTableI3D: FnPtr,
         pub(super) GetGammaTableParametersI3D: FnPtr,
         pub(super) GetGenlockSampleRateI3D: FnPtr,
         pub(super) GetGenlockSourceDelayI3D: FnPtr,
         pub(super) GetGenlockSourceEdgeI3D: FnPtr,
         pub(super) GetGenlockSourceI3D: FnPtr,
         pub(super) GetLayerPaletteEntries: FnPtr,
         pub(super) GetMscRateOML: FnPtr,
         pub(super) GetPbufferDCARB: FnPtr,
         pub(super) GetPbufferDCEXT: FnPtr,
         pub(super) GetPixelFormatAttribfvARB: FnPtr,
         pub(super) GetPixelFormatAttribfvEXT: FnPtr,
         pub(super) GetPixelFormatAttribivARB: FnPtr,
         pub(super) GetPixelFormatAttribivEXT: FnPtr,
         pub(super) GetProcAddress: FnPtr,
         pub(super) GetSwapIntervalEXT: FnPtr,
         pub(super) GetSyncValuesOML: FnPtr,
         pub(super) GetVideoDeviceNV: FnPtr,
         pub(super) GetVideoInfoNV: FnPtr,
         pub(super) IsEnabledFrameLockI3D: FnPtr,
         pub(super) IsEnabledGenlockI3D: FnPtr,
         pub(super) JoinSwapGroupNV: FnPtr,
         pub(super) LoadDisplayColorTableEXT: FnPtr,
         pub(super) LockVideoCaptureDeviceNV: FnPtr,
         pub(super) MakeAssociatedContextCurrentAMD: FnPtr,
         pub(super) MakeContextCurrentARB: FnPtr,
         pub(super) MakeContextCurrentEXT: FnPtr,
         pub(super) MakeCurrent: FnPtr,
         pub(super) QueryCurrentContextNV: FnPtr,
         pub(super) QueryFrameCountNV: FnPtr,
         pub(super) QueryFrameLockMasterI3D: FnPtr,
         pub(super) QueryFrameTrackingI3D: FnPtr,
         pub(super) QueryGenlockMaxSourceDelayI3D: FnPtr,
         pub(super) QueryMaxSwapGroupsNV: FnPtr,
         pub(super) QueryPbufferARB: FnPtr,
         pub(super) QueryPbufferEXT: FnPtr,
         pub(super) QuerySwapGroupNV: FnPtr,
         pub(super) QueryVideoCaptureDeviceNV: FnPtr,
         pub(super) RealizeLayerPalette: FnPtr,
         pub(super) ReleaseImageBufferEventsI3D: FnPtr,
         pub(super) ReleasePbufferDCARB: FnPtr,
         pub(super) ReleasePbufferDCEXT: FnPtr,
         pub(super) ReleaseTexImageARB: FnPtr,
         pub(super) ReleaseVideoCaptureDeviceNV: FnPtr,
         pub(super) ReleaseVideoDeviceNV: FnPtr,
         pub(super) ReleaseVideoImageNV: FnPtr,
         pub(super) ResetFrameCountNV: FnPtr,
         pub(super) RestoreBufferRegionARB: FnPtr,
         pub(super) SaveBufferRegionARB: FnPtr,
         pub(super) SendPbufferToVideoNV: FnPtr,
         pub(super) SetDigitalVideoParametersI3D: FnPtr,
         pub(super) SetGammaTableI3D: FnPtr,
         pub(super) SetGammaTableParametersI3D: FnPtr,
         pub(super) SetLayerPaletteEntries: FnPtr,
         pub(super) SetPbufferAttribARB: FnPtr,
         pub(super) SetStereoEmitterState3DL: FnPtr,
         pub(super) ShareLists: FnPtr,
         pub(super) SwapBuffersMscOML: FnPtr,
         pub(super) SwapIntervalEXT: FnPtr,
         pub(super) SwapLayerBuffers: FnPtr,
         pub(super) SwapLayerBuffersMscOML: FnPtr,
         pub(super) UseFontBitmaps: FnPtr,
         pub(super) UseFontBitmapsA: FnPtr,
         pub(super) UseFontBitmapsW: FnPtr,
         pub(super) UseFontOutlines: FnPtr,
         pub(super) UseFontOutlinesA: FnPtr,
         pub(super) UseFontOutlinesW: FnPtr,
         pub(super) WaitForMscOML: FnPtr,
         pub(super) WaitForSbcOML: FnPtr,
    }


    impl Wgl {

     func!(ChoosePixelFormat, c_int, hDc: HDC, pPfd: *const PIXELFORMATDESCRIPTOR);
     func!(DescribePixelFormat, c_int, hdc: HDC, ipfd: c_int, cjpfd: UINT, ppfd: *const PIXELFORMATDESCRIPTOR);
     func!(GetEnhMetaFilePixelFormat, UINT, hemf: HENHMETAFILE, ppfd: *const PIXELFORMATDESCRIPTOR);
     func!(GetPixelFormat, c_int, hdc: HDC);
     func!(SetPixelFormat, BOOL, hdc: HDC, ipfd: c_int, ppfd: *const PIXELFORMATDESCRIPTOR);
     func!(SwapBuffers, BOOL, hdc: HDC);
     func!(AllocateMemoryNV, *mut c_void, size: GLsizei, readfreq: GLfloat, writefreq: GLfloat, priority: GLfloat);
     func!(AssociateImageBufferEventsI3D, BOOL, hDC: HDC, pEvent: *const HANDLE, pAddress: *const LPVOID, pSize: *const DWORD, count: UINT);
     func!(BeginFrameTrackingI3D, BOOL, );
     func!(BindDisplayColorTableEXT, GLboolean, id: GLushort);
     func!(BindSwapBarrierNV, BOOL, group: GLuint, barrier: GLuint);
     func!(BindTexImageARB, BOOL, hPbuffer: HPBUFFERARB, iBuffer: c_int);
     func!(BindVideoCaptureDeviceNV, BOOL, uVideoSlot: UINT, hDevice: HVIDEOINPUTDEVICENV);
     func!(BindVideoDeviceNV, BOOL, hDc: HDC, uVideoSlot: c_int, hVideoDevice: HVIDEOOUTPUTDEVICENV, piAttribList: *const c_int);
     func!(BindVideoImageNV, BOOL, hVideoDevice: HPVIDEODEV, hPbuffer: HPBUFFERARB, iVideoBuffer: c_int);
     func!(BlitContextFramebufferAMD, VOID, dstCtx: HGLRC, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);
     func!(ChoosePixelFormatARB, BOOL, hdc: HDC, piAttribIList: *const c_int, pfAttribFList: *const FLOAT, nMaxFormats: UINT, piFormats: *mut c_int, nNumFormats: *mut UINT);
     func!(ChoosePixelFormatEXT, BOOL, hdc: HDC, piAttribIList: *const c_int, pfAttribFList: *const FLOAT, nMaxFormats: UINT, piFormats: *mut c_int, nNumFormats: *mut UINT);
     func!(CopyContext, BOOL, hglrcSrc: HGLRC, hglrcDst: HGLRC, mask: UINT);
     func!(CopyImageSubDataNV, BOOL, hSrcRC: HGLRC, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, hDstRC: HGLRC, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, width: GLsizei, height: GLsizei, depth: GLsizei);
     func!(CreateAffinityDCNV, HDC, phGpuList: *const HGPUNV);
     func!(CreateAssociatedContextAMD, HGLRC, id: UINT);
     func!(CreateAssociatedContextAttribsAMD, HGLRC, id: UINT, hShareContext: HGLRC, attribList: *const c_int);
     func!(CreateBufferRegionARB, HANDLE, hDC: HDC, iLayerPlane: c_int, uType: UINT);
     func!(CreateContext, HGLRC, hDc: HDC);
     func!(CreateContextAttribsARB, HGLRC, hDC: HDC, hShareContext: HGLRC, attribList: *const c_int);
     func!(CreateDisplayColorTableEXT, GLboolean, id: GLushort);
     func!(CreateImageBufferI3D, LPVOID, hDC: HDC, dwSize: DWORD, uFlags: UINT);
     func!(CreateLayerContext, HGLRC, hDc: HDC, level: c_int);
     func!(CreatePbufferARB, HPBUFFERARB, hDC: HDC, iPixelFormat: c_int, iWidth: c_int, iHeight: c_int, piAttribList: *const c_int);
     func!(CreatePbufferEXT, HPBUFFEREXT, hDC: HDC, iPixelFormat: c_int, iWidth: c_int, iHeight: c_int, piAttribList: *const c_int);
     func!(DXCloseDeviceNV, BOOL, hDevice: HANDLE);
     func!(DXLockObjectsNV, BOOL, hDevice: HANDLE, count: GLint, hObjects: *mut HANDLE);
     func!(DXObjectAccessNV, BOOL, hObject: HANDLE, access: GLenum);
     func!(DXOpenDeviceNV, HANDLE, dxDevice: *mut c_void);
     func!(DXRegisterObjectNV, HANDLE, hDevice: HANDLE, dxObject: *mut c_void, name: GLuint, type_: GLenum, access: GLenum);
     func!(DXSetResourceShareHandleNV, BOOL, dxObject: *mut c_void, shareHandle: HANDLE);
     func!(DXUnlockObjectsNV, BOOL, hDevice: HANDLE, count: GLint, hObjects: *mut HANDLE);
     func!(DXUnregisterObjectNV, BOOL, hDevice: HANDLE, hObject: HANDLE);
     func!(DelayBeforeSwapNV, BOOL, hDC: HDC, seconds: GLfloat);
     func!(DeleteAssociatedContextAMD, BOOL, hglrc: HGLRC);
     func!(DeleteBufferRegionARB, VOID, hRegion: HANDLE);
     func!(DeleteContext, BOOL, oldContext: HGLRC);
     func!(DeleteDCNV, BOOL, hdc: HDC);
     func!(DescribeLayerPlane, BOOL, hDc: HDC, pixelFormat: c_int, layerPlane: c_int, nBytes: UINT, plpd: *const LAYERPLANEDESCRIPTOR);
     func!(DestroyDisplayColorTableEXT, VOID, id: GLushort);
     func!(DestroyImageBufferI3D, BOOL, hDC: HDC, pAddress: LPVOID);
     func!(DestroyPbufferARB, BOOL, hPbuffer: HPBUFFERARB);
     func!(DestroyPbufferEXT, BOOL, hPbuffer: HPBUFFEREXT);
     func!(DisableFrameLockI3D, BOOL, );
     func!(DisableGenlockI3D, BOOL, hDC: HDC);
     func!(EnableFrameLockI3D, BOOL, );
     func!(EnableGenlockI3D, BOOL, hDC: HDC);
     func!(EndFrameTrackingI3D, BOOL, );
     func!(EnumGpuDevicesNV, BOOL, hGpu: HGPUNV, iDeviceIndex: UINT, lpGpuDevice: PGPU_DEVICE);
     func!(EnumGpusFromAffinityDCNV, BOOL, hAffinityDC: HDC, iGpuIndex: UINT, hGpu: *mut HGPUNV);
     func!(EnumGpusNV, BOOL, iGpuIndex: UINT, phGpu: *mut HGPUNV);
     func!(EnumerateVideoCaptureDevicesNV, UINT, hDc: HDC, phDeviceList: *mut HVIDEOINPUTDEVICENV);
     func!(EnumerateVideoDevicesNV, c_int, hDc: HDC, phDeviceList: *mut HVIDEOOUTPUTDEVICENV);
     func!(FreeMemoryNV, (), pointer: *mut c_void);
     func!(GenlockSampleRateI3D, BOOL, hDC: HDC, uRate: UINT);
     func!(GenlockSourceDelayI3D, BOOL, hDC: HDC, uDelay: UINT);
     func!(GenlockSourceEdgeI3D, BOOL, hDC: HDC, uEdge: UINT);
     func!(GenlockSourceI3D, BOOL, hDC: HDC, uSource: UINT);
     func!(GetContextGPUIDAMD, UINT, hglrc: HGLRC);
     func!(GetCurrentAssociatedContextAMD, HGLRC, );
     func!(GetCurrentContext, HGLRC, );
     func!(GetCurrentDC, HDC, );
     func!(GetCurrentReadDCARB, HDC, );
     func!(GetCurrentReadDCEXT, HDC, );
     func!(GetDigitalVideoParametersI3D, BOOL, hDC: HDC, iAttribute: c_int, piValue: *mut c_int);
     func!(GetExtensionsStringARB, *const c_char, hdc: HDC);
     func!(GetExtensionsStringEXT, *const c_char, );
     func!(GetFrameUsageI3D, BOOL, pUsage: *mut c_float);
     func!(GetGPUIDsAMD, UINT, maxCount: UINT, ids: *mut UINT);
     func!(GetGPUInfoAMD, INT, id: UINT, property: INT, dataType: GLenum, size: UINT, data: *mut c_void);
     func!(GetGammaTableI3D, BOOL, hDC: HDC, iEntries: c_int, puRed: *mut USHORT, puGreen: *mut USHORT, puBlue: *mut USHORT);
     func!(GetGammaTableParametersI3D, BOOL, hDC: HDC, iAttribute: c_int, piValue: *mut c_int);
     func!(GetGenlockSampleRateI3D, BOOL, hDC: HDC, uRate: *mut UINT);
     func!(GetGenlockSourceDelayI3D, BOOL, hDC: HDC, uDelay: *mut UINT);
     func!(GetGenlockSourceEdgeI3D, BOOL, hDC: HDC, uEdge: *mut UINT);
     func!(GetGenlockSourceI3D, BOOL, hDC: HDC, uSource: *mut UINT);
     func!(GetLayerPaletteEntries, c_int, hdc: HDC, iLayerPlane: c_int, iStart: c_int, cEntries: c_int, pcr: *const COLORREF);
     func!(GetMscRateOML, BOOL, hdc: HDC, numerator: *mut INT32, denominator: *mut INT32);
     func!(GetPbufferDCARB, HDC, hPbuffer: HPBUFFERARB);
     func!(GetPbufferDCEXT, HDC, hPbuffer: HPBUFFEREXT);
     func!(GetPixelFormatAttribfvARB, BOOL, hdc: HDC, iPixelFormat: c_int, iLayerPlane: c_int, nAttributes: UINT, piAttributes: *const c_int, pfValues: *mut FLOAT);
     func!(GetPixelFormatAttribfvEXT, BOOL, hdc: HDC, iPixelFormat: c_int, iLayerPlane: c_int, nAttributes: UINT, piAttributes: *mut c_int, pfValues: *mut FLOAT);
     func!(GetPixelFormatAttribivARB, BOOL, hdc: HDC, iPixelFormat: c_int, iLayerPlane: c_int, nAttributes: UINT, piAttributes: *const c_int, piValues: *mut c_int);
     func!(GetPixelFormatAttribivEXT, BOOL, hdc: HDC, iPixelFormat: c_int, iLayerPlane: c_int, nAttributes: UINT, piAttributes: *mut c_int, piValues: *mut c_int);
     func!(GetProcAddress, PROC, lpszProc: LPCSTR);
     func!(GetSwapIntervalEXT, c_int, );
     func!(GetSyncValuesOML, BOOL, hdc: HDC, ust: *mut INT64, msc: *mut INT64, sbc: *mut INT64);
     func!(GetVideoDeviceNV, BOOL, hDC: HDC, numDevices: c_int, hVideoDevice: *mut HPVIDEODEV);
     func!(GetVideoInfoNV, BOOL, hpVideoDevice: HPVIDEODEV, pulCounterOutputPbuffer: *mut c_long, pulCounterOutputVideo: *mut c_long);
     func!(IsEnabledFrameLockI3D, BOOL, pFlag: *mut BOOL);
     func!(IsEnabledGenlockI3D, BOOL, hDC: HDC, pFlag: *mut BOOL);
     func!(JoinSwapGroupNV, BOOL, hDC: HDC, group: GLuint);
     func!(LoadDisplayColorTableEXT, GLboolean, table: *const GLushort, length: GLuint);
     func!(LockVideoCaptureDeviceNV, BOOL, hDc: HDC, hDevice: HVIDEOINPUTDEVICENV);
     func!(MakeAssociatedContextCurrentAMD, BOOL, hglrc: HGLRC);
     func!(MakeContextCurrentARB, BOOL, hDrawDC: HDC, hReadDC: HDC, hglrc: HGLRC);
     func!(MakeContextCurrentEXT, BOOL, hDrawDC: HDC, hReadDC: HDC, hglrc: HGLRC);
     func!(MakeCurrent, BOOL, hDc: HDC, newContext: HGLRC);
     func!(QueryCurrentContextNV, BOOL, iAttribute: c_int, piValue: *mut c_int);
     func!(QueryFrameCountNV, BOOL, hDC: HDC, count: *mut GLuint);
     func!(QueryFrameLockMasterI3D, BOOL, pFlag: *mut BOOL);
     func!(QueryFrameTrackingI3D, BOOL, pFrameCount: *mut DWORD, pMissedFrames: *mut DWORD, pLastMissedUsage: *mut c_float);
     func!(QueryGenlockMaxSourceDelayI3D, BOOL, hDC: HDC, uMaxLineDelay: *mut UINT, uMaxPixelDelay: *mut UINT);
     func!(QueryMaxSwapGroupsNV, BOOL, hDC: HDC, maxGroups: *mut GLuint, maxBarriers: *mut GLuint);
     func!(QueryPbufferARB, BOOL, hPbuffer: HPBUFFERARB, iAttribute: c_int, piValue: *mut c_int);
     func!(QueryPbufferEXT, BOOL, hPbuffer: HPBUFFEREXT, iAttribute: c_int, piValue: *mut c_int);
     func!(QuerySwapGroupNV, BOOL, hDC: HDC, group: *mut GLuint, barrier: *mut GLuint);
     func!(QueryVideoCaptureDeviceNV, BOOL, hDc: HDC, hDevice: HVIDEOINPUTDEVICENV, iAttribute: c_int, piValue: *mut c_int);
     func!(RealizeLayerPalette, BOOL, hdc: HDC, iLayerPlane: c_int, bRealize: BOOL);
     func!(ReleaseImageBufferEventsI3D, BOOL, hDC: HDC, pAddress: *const LPVOID, count: UINT);
     func!(ReleasePbufferDCARB, c_int, hPbuffer: HPBUFFERARB, hDC: HDC);
     func!(ReleasePbufferDCEXT, c_int, hPbuffer: HPBUFFEREXT, hDC: HDC);
     func!(ReleaseTexImageARB, BOOL, hPbuffer: HPBUFFERARB, iBuffer: c_int);
     func!(ReleaseVideoCaptureDeviceNV, BOOL, hDc: HDC, hDevice: HVIDEOINPUTDEVICENV);
     func!(ReleaseVideoDeviceNV, BOOL, hVideoDevice: HPVIDEODEV);
     func!(ReleaseVideoImageNV, BOOL, hPbuffer: HPBUFFERARB, iVideoBuffer: c_int);
     func!(ResetFrameCountNV, BOOL, hDC: HDC);
     func!(RestoreBufferRegionARB, BOOL, hRegion: HANDLE, x: c_int, y: c_int, width: c_int, height: c_int, xSrc: c_int, ySrc: c_int);
     func!(SaveBufferRegionARB, BOOL, hRegion: HANDLE, x: c_int, y: c_int, width: c_int, height: c_int);
     func!(SendPbufferToVideoNV, BOOL, hPbuffer: HPBUFFERARB, iBufferType: c_int, pulCounterPbuffer: *mut c_long, bBlock: BOOL);
     func!(SetDigitalVideoParametersI3D, BOOL, hDC: HDC, iAttribute: c_int, piValue: *const c_int);
     func!(SetGammaTableI3D, BOOL, hDC: HDC, iEntries: c_int, puRed: *const USHORT, puGreen: *const USHORT, puBlue: *const USHORT);
     func!(SetGammaTableParametersI3D, BOOL, hDC: HDC, iAttribute: c_int, piValue: *const c_int);
     func!(SetLayerPaletteEntries, c_int, hdc: HDC, iLayerPlane: c_int, iStart: c_int, cEntries: c_int, pcr: *const COLORREF);
     func!(SetPbufferAttribARB, BOOL, hPbuffer: HPBUFFERARB, piAttribList: *const c_int);
     func!(SetStereoEmitterState3DL, BOOL, hDC: HDC, uState: UINT);
     func!(ShareLists, BOOL, hrcSrvShare: HGLRC, hrcSrvSource: HGLRC);
     func!(SwapBuffersMscOML, INT64, hdc: HDC, target_msc: INT64, divisor: INT64, remainder: INT64);
     func!(SwapIntervalEXT, BOOL, interval: c_int);
     func!(SwapLayerBuffers, BOOL, hdc: HDC, fuFlags: UINT);
     func!(SwapLayerBuffersMscOML, INT64, hdc: HDC, fuPlanes: INT, target_msc: INT64, divisor: INT64, remainder: INT64);
     func!(UseFontBitmaps, BOOL, hDC: HDC, first: DWORD, count: DWORD, listBase: DWORD);
     func!(UseFontBitmapsA, BOOL, hDC: HDC, first: DWORD, count: DWORD, listBase: DWORD);
     func!(UseFontBitmapsW, BOOL, hDC: HDC, first: DWORD, count: DWORD, listBase: DWORD);
     func!(UseFontOutlines, BOOL, hDC: HDC, first: DWORD, count: DWORD, listBase: DWORD, deviation: FLOAT, extrusion: FLOAT, format: c_int, lpgmf: LPGLYPHMETRICSFLOAT);
     func!(UseFontOutlinesA, BOOL, hDC: HDC, first: DWORD, count: DWORD, listBase: DWORD, deviation: FLOAT, extrusion: FLOAT, format: c_int, lpgmf: LPGLYPHMETRICSFLOAT);
     func!(UseFontOutlinesW, BOOL, hDC: HDC, first: DWORD, count: DWORD, listBase: DWORD, deviation: FLOAT, extrusion: FLOAT, format: c_int, lpgmf: LPGLYPHMETRICSFLOAT);
     func!(WaitForMscOML, BOOL, hdc: HDC, target_msc: INT64, divisor: INT64, remainder: INT64, ust: *mut INT64, msc: *mut INT64, sbc: *mut INT64);
     func!(WaitForSbcOML, BOOL, hdc: HDC, target_sbc: INT64, ust: *mut INT64, msc: *mut INT64, sbc: *mut INT64);

    }
}


pub fn load<F>(mut loadfn: F) -> functions::Wgl where F: FnMut(&'static str) -> *const c_void {
    #[allow(unused_mut)]
    let mut ctx = Wgl {
         ChoosePixelFormat: FnPtr::new(loadfn("ChoosePixelFormat")),
         DescribePixelFormat: FnPtr::new(loadfn("DescribePixelFormat")),
         GetEnhMetaFilePixelFormat: FnPtr::new(loadfn("GetEnhMetaFilePixelFormat")),
         GetPixelFormat: FnPtr::new(loadfn("GetPixelFormat")),
         SetPixelFormat: FnPtr::new(loadfn("SetPixelFormat")),
         SwapBuffers: FnPtr::new(loadfn("SwapBuffers")),
         AllocateMemoryNV: FnPtr::new(loadfn("wglAllocateMemoryNV")),
         AssociateImageBufferEventsI3D: FnPtr::new(loadfn("wglAssociateImageBufferEventsI3D")),
         BeginFrameTrackingI3D: FnPtr::new(loadfn("wglBeginFrameTrackingI3D")),
         BindDisplayColorTableEXT: FnPtr::new(loadfn("wglBindDisplayColorTableEXT")),
         BindSwapBarrierNV: FnPtr::new(loadfn("wglBindSwapBarrierNV")),
         BindTexImageARB: FnPtr::new(loadfn("wglBindTexImageARB")),
         BindVideoCaptureDeviceNV: FnPtr::new(loadfn("wglBindVideoCaptureDeviceNV")),
         BindVideoDeviceNV: FnPtr::new(loadfn("wglBindVideoDeviceNV")),
         BindVideoImageNV: FnPtr::new(loadfn("wglBindVideoImageNV")),
         BlitContextFramebufferAMD: FnPtr::new(loadfn("wglBlitContextFramebufferAMD")),
         ChoosePixelFormatARB: FnPtr::new(loadfn("wglChoosePixelFormatARB")),
         ChoosePixelFormatEXT: FnPtr::new(loadfn("wglChoosePixelFormatEXT")),
         CopyContext: FnPtr::new(loadfn("wglCopyContext")),
         CopyImageSubDataNV: FnPtr::new(loadfn("wglCopyImageSubDataNV")),
         CreateAffinityDCNV: FnPtr::new(loadfn("wglCreateAffinityDCNV")),
         CreateAssociatedContextAMD: FnPtr::new(loadfn("wglCreateAssociatedContextAMD")),
         CreateAssociatedContextAttribsAMD: FnPtr::new(loadfn("wglCreateAssociatedContextAttribsAMD")),
         CreateBufferRegionARB: FnPtr::new(loadfn("wglCreateBufferRegionARB")),
         CreateContext: FnPtr::new(loadfn("wglCreateContext")),
         CreateContextAttribsARB: FnPtr::new(loadfn("wglCreateContextAttribsARB")),
         CreateDisplayColorTableEXT: FnPtr::new(loadfn("wglCreateDisplayColorTableEXT")),
         CreateImageBufferI3D: FnPtr::new(loadfn("wglCreateImageBufferI3D")),
         CreateLayerContext: FnPtr::new(loadfn("wglCreateLayerContext")),
         CreatePbufferARB: FnPtr::new(loadfn("wglCreatePbufferARB")),
         CreatePbufferEXT: FnPtr::new(loadfn("wglCreatePbufferEXT")),
         DXCloseDeviceNV: FnPtr::new(loadfn("wglDXCloseDeviceNV")),
         DXLockObjectsNV: FnPtr::new(loadfn("wglDXLockObjectsNV")),
         DXObjectAccessNV: FnPtr::new(loadfn("wglDXObjectAccessNV")),
         DXOpenDeviceNV: FnPtr::new(loadfn("wglDXOpenDeviceNV")),
         DXRegisterObjectNV: FnPtr::new(loadfn("wglDXRegisterObjectNV")),
         DXSetResourceShareHandleNV: FnPtr::new(loadfn("wglDXSetResourceShareHandleNV")),
         DXUnlockObjectsNV: FnPtr::new(loadfn("wglDXUnlockObjectsNV")),
         DXUnregisterObjectNV: FnPtr::new(loadfn("wglDXUnregisterObjectNV")),
         DelayBeforeSwapNV: FnPtr::new(loadfn("wglDelayBeforeSwapNV")),
         DeleteAssociatedContextAMD: FnPtr::new(loadfn("wglDeleteAssociatedContextAMD")),
         DeleteBufferRegionARB: FnPtr::new(loadfn("wglDeleteBufferRegionARB")),
         DeleteContext: FnPtr::new(loadfn("wglDeleteContext")),
         DeleteDCNV: FnPtr::new(loadfn("wglDeleteDCNV")),
         DescribeLayerPlane: FnPtr::new(loadfn("wglDescribeLayerPlane")),
         DestroyDisplayColorTableEXT: FnPtr::new(loadfn("wglDestroyDisplayColorTableEXT")),
         DestroyImageBufferI3D: FnPtr::new(loadfn("wglDestroyImageBufferI3D")),
         DestroyPbufferARB: FnPtr::new(loadfn("wglDestroyPbufferARB")),
         DestroyPbufferEXT: FnPtr::new(loadfn("wglDestroyPbufferEXT")),
         DisableFrameLockI3D: FnPtr::new(loadfn("wglDisableFrameLockI3D")),
         DisableGenlockI3D: FnPtr::new(loadfn("wglDisableGenlockI3D")),
         EnableFrameLockI3D: FnPtr::new(loadfn("wglEnableFrameLockI3D")),
         EnableGenlockI3D: FnPtr::new(loadfn("wglEnableGenlockI3D")),
         EndFrameTrackingI3D: FnPtr::new(loadfn("wglEndFrameTrackingI3D")),
         EnumGpuDevicesNV: FnPtr::new(loadfn("wglEnumGpuDevicesNV")),
         EnumGpusFromAffinityDCNV: FnPtr::new(loadfn("wglEnumGpusFromAffinityDCNV")),
         EnumGpusNV: FnPtr::new(loadfn("wglEnumGpusNV")),
         EnumerateVideoCaptureDevicesNV: FnPtr::new(loadfn("wglEnumerateVideoCaptureDevicesNV")),
         EnumerateVideoDevicesNV: FnPtr::new(loadfn("wglEnumerateVideoDevicesNV")),
         FreeMemoryNV: FnPtr::new(loadfn("wglFreeMemoryNV")),
         GenlockSampleRateI3D: FnPtr::new(loadfn("wglGenlockSampleRateI3D")),
         GenlockSourceDelayI3D: FnPtr::new(loadfn("wglGenlockSourceDelayI3D")),
         GenlockSourceEdgeI3D: FnPtr::new(loadfn("wglGenlockSourceEdgeI3D")),
         GenlockSourceI3D: FnPtr::new(loadfn("wglGenlockSourceI3D")),
         GetContextGPUIDAMD: FnPtr::new(loadfn("wglGetContextGPUIDAMD")),
         GetCurrentAssociatedContextAMD: FnPtr::new(loadfn("wglGetCurrentAssociatedContextAMD")),
         GetCurrentContext: FnPtr::new(loadfn("wglGetCurrentContext")),
         GetCurrentDC: FnPtr::new(loadfn("wglGetCurrentDC")),
         GetCurrentReadDCARB: FnPtr::new(loadfn("wglGetCurrentReadDCARB")),
         GetCurrentReadDCEXT: FnPtr::new(loadfn("wglGetCurrentReadDCEXT")),
         GetDigitalVideoParametersI3D: FnPtr::new(loadfn("wglGetDigitalVideoParametersI3D")),
         GetExtensionsStringARB: FnPtr::new(loadfn("wglGetExtensionsStringARB")),
         GetExtensionsStringEXT: FnPtr::new(loadfn("wglGetExtensionsStringEXT")),
         GetFrameUsageI3D: FnPtr::new(loadfn("wglGetFrameUsageI3D")),
         GetGPUIDsAMD: FnPtr::new(loadfn("wglGetGPUIDsAMD")),
         GetGPUInfoAMD: FnPtr::new(loadfn("wglGetGPUInfoAMD")),
         GetGammaTableI3D: FnPtr::new(loadfn("wglGetGammaTableI3D")),
         GetGammaTableParametersI3D: FnPtr::new(loadfn("wglGetGammaTableParametersI3D")),
         GetGenlockSampleRateI3D: FnPtr::new(loadfn("wglGetGenlockSampleRateI3D")),
         GetGenlockSourceDelayI3D: FnPtr::new(loadfn("wglGetGenlockSourceDelayI3D")),
         GetGenlockSourceEdgeI3D: FnPtr::new(loadfn("wglGetGenlockSourceEdgeI3D")),
         GetGenlockSourceI3D: FnPtr::new(loadfn("wglGetGenlockSourceI3D")),
         GetLayerPaletteEntries: FnPtr::new(loadfn("wglGetLayerPaletteEntries")),
         GetMscRateOML: FnPtr::new(loadfn("wglGetMscRateOML")),
         GetPbufferDCARB: FnPtr::new(loadfn("wglGetPbufferDCARB")),
         GetPbufferDCEXT: FnPtr::new(loadfn("wglGetPbufferDCEXT")),
         GetPixelFormatAttribfvARB: FnPtr::new(loadfn("wglGetPixelFormatAttribfvARB")),
         GetPixelFormatAttribfvEXT: FnPtr::new(loadfn("wglGetPixelFormatAttribfvEXT")),
         GetPixelFormatAttribivARB: FnPtr::new(loadfn("wglGetPixelFormatAttribivARB")),
         GetPixelFormatAttribivEXT: FnPtr::new(loadfn("wglGetPixelFormatAttribivEXT")),
         GetProcAddress: FnPtr::new(loadfn("wglGetProcAddress")),
         GetSwapIntervalEXT: FnPtr::new(loadfn("wglGetSwapIntervalEXT")),
         GetSyncValuesOML: FnPtr::new(loadfn("wglGetSyncValuesOML")),
         GetVideoDeviceNV: FnPtr::new(loadfn("wglGetVideoDeviceNV")),
         GetVideoInfoNV: FnPtr::new(loadfn("wglGetVideoInfoNV")),
         IsEnabledFrameLockI3D: FnPtr::new(loadfn("wglIsEnabledFrameLockI3D")),
         IsEnabledGenlockI3D: FnPtr::new(loadfn("wglIsEnabledGenlockI3D")),
         JoinSwapGroupNV: FnPtr::new(loadfn("wglJoinSwapGroupNV")),
         LoadDisplayColorTableEXT: FnPtr::new(loadfn("wglLoadDisplayColorTableEXT")),
         LockVideoCaptureDeviceNV: FnPtr::new(loadfn("wglLockVideoCaptureDeviceNV")),
         MakeAssociatedContextCurrentAMD: FnPtr::new(loadfn("wglMakeAssociatedContextCurrentAMD")),
         MakeContextCurrentARB: FnPtr::new(loadfn("wglMakeContextCurrentARB")),
         MakeContextCurrentEXT: FnPtr::new(loadfn("wglMakeContextCurrentEXT")),
         MakeCurrent: FnPtr::new(loadfn("wglMakeCurrent")),
         QueryCurrentContextNV: FnPtr::new(loadfn("wglQueryCurrentContextNV")),
         QueryFrameCountNV: FnPtr::new(loadfn("wglQueryFrameCountNV")),
         QueryFrameLockMasterI3D: FnPtr::new(loadfn("wglQueryFrameLockMasterI3D")),
         QueryFrameTrackingI3D: FnPtr::new(loadfn("wglQueryFrameTrackingI3D")),
         QueryGenlockMaxSourceDelayI3D: FnPtr::new(loadfn("wglQueryGenlockMaxSourceDelayI3D")),
         QueryMaxSwapGroupsNV: FnPtr::new(loadfn("wglQueryMaxSwapGroupsNV")),
         QueryPbufferARB: FnPtr::new(loadfn("wglQueryPbufferARB")),
         QueryPbufferEXT: FnPtr::new(loadfn("wglQueryPbufferEXT")),
         QuerySwapGroupNV: FnPtr::new(loadfn("wglQuerySwapGroupNV")),
         QueryVideoCaptureDeviceNV: FnPtr::new(loadfn("wglQueryVideoCaptureDeviceNV")),
         RealizeLayerPalette: FnPtr::new(loadfn("wglRealizeLayerPalette")),
         ReleaseImageBufferEventsI3D: FnPtr::new(loadfn("wglReleaseImageBufferEventsI3D")),
         ReleasePbufferDCARB: FnPtr::new(loadfn("wglReleasePbufferDCARB")),
         ReleasePbufferDCEXT: FnPtr::new(loadfn("wglReleasePbufferDCEXT")),
         ReleaseTexImageARB: FnPtr::new(loadfn("wglReleaseTexImageARB")),
         ReleaseVideoCaptureDeviceNV: FnPtr::new(loadfn("wglReleaseVideoCaptureDeviceNV")),
         ReleaseVideoDeviceNV: FnPtr::new(loadfn("wglReleaseVideoDeviceNV")),
         ReleaseVideoImageNV: FnPtr::new(loadfn("wglReleaseVideoImageNV")),
         ResetFrameCountNV: FnPtr::new(loadfn("wglResetFrameCountNV")),
         RestoreBufferRegionARB: FnPtr::new(loadfn("wglRestoreBufferRegionARB")),
         SaveBufferRegionARB: FnPtr::new(loadfn("wglSaveBufferRegionARB")),
         SendPbufferToVideoNV: FnPtr::new(loadfn("wglSendPbufferToVideoNV")),
         SetDigitalVideoParametersI3D: FnPtr::new(loadfn("wglSetDigitalVideoParametersI3D")),
         SetGammaTableI3D: FnPtr::new(loadfn("wglSetGammaTableI3D")),
         SetGammaTableParametersI3D: FnPtr::new(loadfn("wglSetGammaTableParametersI3D")),
         SetLayerPaletteEntries: FnPtr::new(loadfn("wglSetLayerPaletteEntries")),
         SetPbufferAttribARB: FnPtr::new(loadfn("wglSetPbufferAttribARB")),
         SetStereoEmitterState3DL: FnPtr::new(loadfn("wglSetStereoEmitterState3DL")),
         ShareLists: FnPtr::new(loadfn("wglShareLists")),
         SwapBuffersMscOML: FnPtr::new(loadfn("wglSwapBuffersMscOML")),
         SwapIntervalEXT: FnPtr::new(loadfn("wglSwapIntervalEXT")),
         SwapLayerBuffers: FnPtr::new(loadfn("wglSwapLayerBuffers")),
         SwapLayerBuffersMscOML: FnPtr::new(loadfn("wglSwapLayerBuffersMscOML")),
         UseFontBitmaps: FnPtr::new(loadfn("wglUseFontBitmaps")),
         UseFontBitmapsA: FnPtr::new(loadfn("wglUseFontBitmapsA")),
         UseFontBitmapsW: FnPtr::new(loadfn("wglUseFontBitmapsW")),
         UseFontOutlines: FnPtr::new(loadfn("wglUseFontOutlines")),
         UseFontOutlinesA: FnPtr::new(loadfn("wglUseFontOutlinesA")),
         UseFontOutlinesW: FnPtr::new(loadfn("wglUseFontOutlinesW")),
         WaitForMscOML: FnPtr::new(loadfn("wglWaitForMscOML")),
         WaitForSbcOML: FnPtr::new(loadfn("wglWaitForSbcOML")),
    };


     ctx
}


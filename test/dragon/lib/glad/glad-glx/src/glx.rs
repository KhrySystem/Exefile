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
    fn not_initialized() -> ! { panic!("glx: function not initialized") }
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

pub type XID = std::os::raw::c_ulong;
pub type Bool = std::os::raw::c_int;
pub enum Display {}

pub type Font = XID;
pub type Pixmap = XID;
pub type Colormap = XID;
pub type Status = XID;
pub enum Visual {}
pub type VisualID = std::os::raw::c_ulong;
pub type Window = XID;
pub type GLXFBConfigID = XID;
pub type GLXFBConfig = *const std::os::raw::c_void;
pub type GLXContextID = XID;
pub type GLXContext = *const std::os::raw::c_void;
pub type GLXPixmap = XID;
pub type GLXDrawable = XID;
pub type GLXWindow = XID;
pub type GLXPbuffer = XID;
pub enum __GLXextFuncPtr_fn {}
pub type __GLXextFuncPtr = *mut __GLXextFuncPtr_fn;
pub type GLXVideoCaptureDeviceNV = XID;
pub type GLXVideoDeviceNV = std::os::raw::c_int;
pub type GLXVideoSourceSGIX = XID;
pub type GLXFBConfigIDSGIX = XID;
pub type GLXFBConfigSGIX = *const std::os::raw::c_void;
pub type GLXPbufferSGIX = XID;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XVisualInfo {
    pub visual: *mut Visual,
    pub visualid: VisualID,
    pub screen: std::os::raw::c_int,
    pub depth: std::os::raw::c_int,
    pub class: std::os::raw::c_int,
    pub red_mask: std::os::raw::c_ulong,
    pub green_mask: std::os::raw::c_ulong,
    pub blue_mask: std::os::raw::c_ulong,
    pub colormap_size: std::os::raw::c_int,
    pub bits_per_rgb: std::os::raw::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GLXPbufferClobberEvent {
    pub event_type: std::os::raw::c_int,
    pub draw_type: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: Bool,
    pub display: *const Display,
    pub drawable: GLXDrawable,
    pub buffer_mask: std::os::raw::c_uint,
    pub aux_buffer: std::os::raw::c_uint,
    pub x: std::os::raw::c_int,
    pub y: std::os::raw::c_int,
    pub width: std::os::raw::c_int,
    pub height: std::os::raw::c_int,
    pub count: std::os::raw::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GLXBufferSwapComplete {
    pub type_: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: Bool,
    pub display: *const Display,
    pub drawable: GLXDrawable,
    pub event_type: std::os::raw::c_int,
    pub ust: i64,
    pub msc: i64,
    pub sbc: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GLXBufferClobberEventSGIX {
    pub type_: std::os::raw::c_int,
    pub serial: std::os::raw::c_ulong,
    pub send_event: Bool,
    pub display: *const Display,
    pub drawable: GLXDrawable,
    pub event_type: std::os::raw::c_int,
    pub draw_type: std::os::raw::c_int,
    pub mask: std::os::raw::c_uint,
    pub x: std::os::raw::c_int,
    pub y: std::os::raw::c_int,
    pub width: std::os::raw::c_int,
    pub height: std::os::raw::c_int,
    pub count: std::os::raw::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GLXHyperpipeNetworkSGIX {
    pub pipeName: [std::os::raw::c_char; super::enumerations::HYPERPIPE_PIPE_NAME_LENGTH_SGIX as usize],
    pub networkId: std::os::raw::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GLXHyperpipeConfigSGIX {
    pub pipeName: [std::os::raw::c_char; super::enumerations::HYPERPIPE_PIPE_NAME_LENGTH_SGIX as usize],
    pub channel: std::os::raw::c_int,
    pub participationType: std::os::raw::c_uint,
    pub timeSlice: std::os::raw::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GLXPipeRect {
    pub pipeName: [std::os::raw::c_char; super::enumerations::HYPERPIPE_PIPE_NAME_LENGTH_SGIX as usize],
    pub srcXOrigin: std::os::raw::c_int,
    pub srcYOrigin: std::os::raw::c_int,
    pub srcWidth: std::os::raw::c_int,
    pub srcHeight: std::os::raw::c_int,
    pub destXOrigin: std::os::raw::c_int,
    pub destYOrigin: std::os::raw::c_int,
    pub destWidth: std::os::raw::c_int,
    pub destHeight: std::os::raw::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GLXPipeRectLimits {
    pub pipeName: [std::os::raw::c_char; super::enumerations::HYPERPIPE_PIPE_NAME_LENGTH_SGIX as usize],
    pub XOrigin: std::os::raw::c_int,
    pub YOrigin: std::os::raw::c_int,
    pub maxHeight: std::os::raw::c_int,
    pub maxWidth: std::os::raw::c_int,
}}

pub mod enumerations {
    #![allow(dead_code, non_upper_case_globals, unused_imports)]

    use std::os::raw::*;
    use super::types::*;

    pub const _3DFX_FULLSCREEN_MODE_MESA: c_uint = 0x2;
    pub const _3DFX_WINDOW_MODE_MESA: c_uint = 0x1;
    pub const ACCUM_ALPHA_SIZE: c_uint = 17;
    pub const ACCUM_BLUE_SIZE: c_uint = 16;
    pub const ACCUM_BUFFER_BIT: c_uint = 0x00000080;
    pub const ACCUM_BUFFER_BIT_SGIX: c_uint = 0x00000080;
    pub const ACCUM_GREEN_SIZE: c_uint = 15;
    pub const ACCUM_RED_SIZE: c_uint = 14;
    pub const ALPHA_SIZE: c_uint = 11;
    pub const AUX0_EXT: c_uint = 0x20E2;
    pub const AUX1_EXT: c_uint = 0x20E3;
    pub const AUX2_EXT: c_uint = 0x20E4;
    pub const AUX3_EXT: c_uint = 0x20E5;
    pub const AUX4_EXT: c_uint = 0x20E6;
    pub const AUX5_EXT: c_uint = 0x20E7;
    pub const AUX6_EXT: c_uint = 0x20E8;
    pub const AUX7_EXT: c_uint = 0x20E9;
    pub const AUX8_EXT: c_uint = 0x20EA;
    pub const AUX9_EXT: c_uint = 0x20EB;
    pub const AUX_BUFFERS: c_uint = 7;
    pub const AUX_BUFFERS_BIT: c_uint = 0x00000010;
    pub const AUX_BUFFERS_BIT_SGIX: c_uint = 0x00000010;
    pub const BACK_BUFFER_AGE_EXT: c_uint = 0x20F4;
    pub const BACK_EXT: c_uint = 0x20E0;
    pub const BACK_LEFT_BUFFER_BIT: c_uint = 0x00000004;
    pub const BACK_LEFT_BUFFER_BIT_SGIX: c_uint = 0x00000004;
    pub const BACK_LEFT_EXT: c_uint = 0x20E0;
    pub const BACK_RIGHT_BUFFER_BIT: c_uint = 0x00000008;
    pub const BACK_RIGHT_BUFFER_BIT_SGIX: c_uint = 0x00000008;
    pub const BACK_RIGHT_EXT: c_uint = 0x20E1;
    pub const BAD_ATTRIBUTE: c_uint = 2;
    pub const BAD_CONTEXT: c_uint = 5;
    pub const BAD_ENUM: c_uint = 7;
    pub const BAD_HYPERPIPE_CONFIG_SGIX: c_uint = 91;
    pub const BAD_HYPERPIPE_SGIX: c_uint = 92;
    pub const BAD_SCREEN: c_uint = 1;
    pub const BAD_VALUE: c_uint = 6;
    pub const BAD_VISUAL: c_uint = 4;
    pub const BIND_TO_MIPMAP_TEXTURE_EXT: c_uint = 0x20D2;
    pub const BIND_TO_TEXTURE_RGBA_EXT: c_uint = 0x20D1;
    pub const BIND_TO_TEXTURE_RGB_EXT: c_uint = 0x20D0;
    pub const BIND_TO_TEXTURE_TARGETS_EXT: c_uint = 0x20D3;
    pub const BLENDED_RGBA_SGIS: c_uint = 0x8025;
    pub const BLUE_SIZE: c_uint = 10;
    pub const BUFFER_CLOBBER_MASK_SGIX: c_uint = 0x08000000;
    pub const BUFFER_SIZE: c_uint = 2;
    pub const BUFFER_SWAP_COMPLETE_INTEL_MASK: c_uint = 0x04000000;
    pub const BufferSwapComplete: c_uint = 1;
    pub const COLOR_INDEX_BIT: c_uint = 0x00000002;
    pub const COLOR_INDEX_BIT_SGIX: c_uint = 0x00000002;
    pub const COLOR_INDEX_TYPE: c_uint = 0x8015;
    pub const COLOR_INDEX_TYPE_SGIX: c_uint = 0x8015;
    pub const COLOR_SAMPLES_NV: c_uint = 0x20B3;
    pub const CONFIG_CAVEAT: c_uint = 0x20;
    pub const CONTEXT_ALLOW_BUFFER_BYTE_ORDER_MISMATCH_ARB: c_uint = 0x2095;
    pub const CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: c_uint = 0x00000002;
    pub const CONTEXT_CORE_PROFILE_BIT_ARB: c_uint = 0x00000001;
    pub const CONTEXT_DEBUG_BIT_ARB: c_uint = 0x00000001;
    pub const CONTEXT_ES2_PROFILE_BIT_EXT: c_uint = 0x00000004;
    pub const CONTEXT_ES_PROFILE_BIT_EXT: c_uint = 0x00000004;
    pub const CONTEXT_FLAGS_ARB: c_uint = 0x2094;
    pub const CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: c_uint = 0x00000002;
    pub const CONTEXT_MAJOR_VERSION_ARB: c_uint = 0x2091;
    pub const CONTEXT_MINOR_VERSION_ARB: c_uint = 0x2092;
    pub const CONTEXT_MULTIGPU_ATTRIB_AFR_NV: c_uint = 0x20AC;
    pub const CONTEXT_MULTIGPU_ATTRIB_MULTICAST_NV: c_uint = 0x20AD;
    pub const CONTEXT_MULTIGPU_ATTRIB_MULTI_DISPLAY_MULTICAST_NV: c_uint = 0x20AE;
    pub const CONTEXT_MULTIGPU_ATTRIB_NV: c_uint = 0x20AA;
    pub const CONTEXT_MULTIGPU_ATTRIB_SINGLE_NV: c_uint = 0x20AB;
    pub const CONTEXT_OPENGL_NO_ERROR_ARB: c_uint = 0x31B3;
    pub const CONTEXT_PRIORITY_HIGH_EXT: c_uint = 0x3101;
    pub const CONTEXT_PRIORITY_LEVEL_EXT: c_uint = 0x3100;
    pub const CONTEXT_PRIORITY_LOW_EXT: c_uint = 0x3103;
    pub const CONTEXT_PRIORITY_MEDIUM_EXT: c_uint = 0x3102;
    pub const CONTEXT_PROFILE_MASK_ARB: c_uint = 0x9126;
    pub const CONTEXT_RELEASE_BEHAVIOR_ARB: c_uint = 0x2097;
    pub const CONTEXT_RELEASE_BEHAVIOR_FLUSH_ARB: c_uint = 0x2098;
    pub const CONTEXT_RELEASE_BEHAVIOR_NONE_ARB: c_uint = 0;
    pub const CONTEXT_RESET_ISOLATION_BIT_ARB: c_uint = 0x00000008;
    pub const CONTEXT_RESET_NOTIFICATION_STRATEGY_ARB: c_uint = 0x8256;
    pub const CONTEXT_ROBUST_ACCESS_BIT_ARB: c_uint = 0x00000004;
    pub const COPY_COMPLETE_INTEL: c_uint = 0x8181;
    pub const COVERAGE_SAMPLES_NV: c_uint = 100001;
    pub const DAMAGED: c_uint = 0x8020;
    pub const DAMAGED_SGIX: c_uint = 0x8020;
    pub const DEPTH_BUFFER_BIT: c_uint = 0x00000020;
    pub const DEPTH_BUFFER_BIT_SGIX: c_uint = 0x00000020;
    pub const DEPTH_SIZE: c_uint = 12;
    pub const DEVICE_ID_NV: c_uint = 0x20CD;
    pub const DIRECT_COLOR: c_uint = 0x8003;
    pub const DIRECT_COLOR_EXT: c_uint = 0x8003;
    pub const DONT_CARE: c_uint = 0xFFFFFFFF;
    pub const DOUBLEBUFFER: c_uint = 5;
    pub const DRAWABLE_TYPE: c_uint = 0x8010;
    pub const DRAWABLE_TYPE_SGIX: c_uint = 0x8010;
    pub const EVENT_MASK: c_uint = 0x801F;
    pub const EVENT_MASK_SGIX: c_uint = 0x801F;
    pub const EXCHANGE_COMPLETE_INTEL: c_uint = 0x8180;
    pub const EXTENSIONS: c_uint = 0x3;
    pub const EXTENSION_NAME: &str = "GLX\0";
    pub const FBCONFIG_ID: c_uint = 0x8013;
    pub const FBCONFIG_ID_SGIX: c_uint = 0x8013;
    pub const FLIP_COMPLETE_INTEL: c_uint = 0x8182;
    pub const FLOAT_COMPONENTS_NV: c_uint = 0x20B0;
    pub const FRAMEBUFFER_SRGB_CAPABLE_ARB: c_uint = 0x20B2;
    pub const FRAMEBUFFER_SRGB_CAPABLE_EXT: c_uint = 0x20B2;
    pub const FRONT_EXT: c_uint = 0x20DE;
    pub const FRONT_LEFT_BUFFER_BIT: c_uint = 0x00000001;
    pub const FRONT_LEFT_BUFFER_BIT_SGIX: c_uint = 0x00000001;
    pub const FRONT_LEFT_EXT: c_uint = 0x20DE;
    pub const FRONT_RIGHT_BUFFER_BIT: c_uint = 0x00000002;
    pub const FRONT_RIGHT_BUFFER_BIT_SGIX: c_uint = 0x00000002;
    pub const FRONT_RIGHT_EXT: c_uint = 0x20DF;
    pub const GENERATE_RESET_ON_VIDEO_MEMORY_PURGE_NV: c_uint = 0x20F7;
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
    pub const GRAY_SCALE: c_uint = 0x8006;
    pub const GRAY_SCALE_EXT: c_uint = 0x8006;
    pub const GREEN_SIZE: c_uint = 9;
    pub const HEIGHT: c_uint = 0x801E;
    pub const HEIGHT_SGIX: c_uint = 0x801E;
    pub const HYPERPIPE_DISPLAY_PIPE_SGIX: c_uint = 0x00000001;
    pub const HYPERPIPE_ID_SGIX: c_uint = 0x8030;
    pub const HYPERPIPE_PIPE_NAME_LENGTH_SGIX: c_uint = 80;
    pub const HYPERPIPE_PIXEL_AVERAGE_SGIX: c_uint = 0x00000004;
    pub const HYPERPIPE_RENDER_PIPE_SGIX: c_uint = 0x00000002;
    pub const HYPERPIPE_STEREO_SGIX: c_uint = 0x00000003;
    pub const LARGEST_PBUFFER: c_uint = 0x801C;
    pub const LARGEST_PBUFFER_SGIX: c_uint = 0x801C;
    pub const LATE_SWAPS_TEAR_EXT: c_uint = 0x20F3;
    pub const LEVEL: c_uint = 3;
    pub const LOSE_CONTEXT_ON_RESET_ARB: c_uint = 0x8252;
    pub const MAX_PBUFFER_HEIGHT: c_uint = 0x8017;
    pub const MAX_PBUFFER_HEIGHT_SGIX: c_uint = 0x8017;
    pub const MAX_PBUFFER_PIXELS: c_uint = 0x8018;
    pub const MAX_PBUFFER_PIXELS_SGIX: c_uint = 0x8018;
    pub const MAX_PBUFFER_WIDTH: c_uint = 0x8016;
    pub const MAX_PBUFFER_WIDTH_SGIX: c_uint = 0x8016;
    pub const MAX_SWAP_INTERVAL_EXT: c_uint = 0x20F2;
    pub const MIPMAP_TEXTURE_EXT: c_uint = 0x20D7;
    pub const MULTISAMPLE_SUB_RECT_HEIGHT_SGIS: c_uint = 0x8027;
    pub const MULTISAMPLE_SUB_RECT_WIDTH_SGIS: c_uint = 0x8026;
    pub const NONE: c_uint = 0x8000;
    pub const NONE_EXT: c_uint = 0x8000;
    pub const NON_CONFORMANT_CONFIG: c_uint = 0x800D;
    pub const NON_CONFORMANT_VISUAL_EXT: c_uint = 0x800D;
    pub const NO_EXTENSION: c_uint = 3;
    pub const NO_RESET_NOTIFICATION_ARB: c_uint = 0x8261;
    pub const NUM_VIDEO_CAPTURE_SLOTS_NV: c_uint = 0x20CF;
    pub const NUM_VIDEO_SLOTS_NV: c_uint = 0x20F0;
    pub const OPTIMAL_PBUFFER_HEIGHT_SGIX: c_uint = 0x801A;
    pub const OPTIMAL_PBUFFER_WIDTH_SGIX: c_uint = 0x8019;
    pub const PBUFFER: c_uint = 0x8023;
    pub const PBUFFER_BIT: c_uint = 0x00000004;
    pub const PBUFFER_BIT_SGIX: c_uint = 0x00000004;
    pub const PBUFFER_CLOBBER_MASK: c_uint = 0x08000000;
    pub const PBUFFER_HEIGHT: c_uint = 0x8040;
    pub const PBUFFER_SGIX: c_uint = 0x8023;
    pub const PBUFFER_WIDTH: c_uint = 0x8041;
    pub const PIPE_RECT_LIMITS_SGIX: c_uint = 0x00000002;
    pub const PIPE_RECT_SGIX: c_uint = 0x00000001;
    pub const PIXMAP_BIT: c_uint = 0x00000002;
    pub const PIXMAP_BIT_SGIX: c_uint = 0x00000002;
    pub const PRESERVED_CONTENTS: c_uint = 0x801B;
    pub const PRESERVED_CONTENTS_SGIX: c_uint = 0x801B;
    pub const PSEUDO_COLOR: c_uint = 0x8004;
    pub const PSEUDO_COLOR_EXT: c_uint = 0x8004;
    pub const PbufferClobber: c_uint = 0;
    pub const RED_SIZE: c_uint = 8;
    pub const RENDERER_ACCELERATED_MESA: c_uint = 0x8186;
    pub const RENDERER_DEVICE_ID_MESA: c_uint = 0x8184;
    pub const RENDERER_OPENGL_COMPATIBILITY_PROFILE_VERSION_MESA: c_uint = 0x818B;
    pub const RENDERER_OPENGL_CORE_PROFILE_VERSION_MESA: c_uint = 0x818A;
    pub const RENDERER_OPENGL_ES2_PROFILE_VERSION_MESA: c_uint = 0x818D;
    pub const RENDERER_OPENGL_ES_PROFILE_VERSION_MESA: c_uint = 0x818C;
    pub const RENDERER_PREFERRED_PROFILE_MESA: c_uint = 0x8189;
    pub const RENDERER_UNIFIED_MEMORY_ARCHITECTURE_MESA: c_uint = 0x8188;
    pub const RENDERER_VENDOR_ID_MESA: c_uint = 0x8183;
    pub const RENDERER_VERSION_MESA: c_uint = 0x8185;
    pub const RENDERER_VIDEO_MEMORY_MESA: c_uint = 0x8187;
    pub const RENDER_TYPE: c_uint = 0x8011;
    pub const RENDER_TYPE_SGIX: c_uint = 0x8011;
    pub const RGBA: c_uint = 4;
    pub const RGBA_BIT: c_uint = 0x00000001;
    pub const RGBA_BIT_SGIX: c_uint = 0x00000001;
    pub const RGBA_FLOAT_BIT_ARB: c_uint = 0x00000004;
    pub const RGBA_FLOAT_TYPE_ARB: c_uint = 0x20B9;
    pub const RGBA_TYPE: c_uint = 0x8014;
    pub const RGBA_TYPE_SGIX: c_uint = 0x8014;
    pub const RGBA_UNSIGNED_FLOAT_BIT_EXT: c_uint = 0x00000008;
    pub const RGBA_UNSIGNED_FLOAT_TYPE_EXT: c_uint = 0x20B1;
    pub const SAMPLES: c_uint = 100001;
    pub const SAMPLES_3DFX: c_uint = 0x8051;
    pub const SAMPLES_ARB: c_uint = 100001;
    pub const SAMPLES_SGIS: c_uint = 100001;
    pub const SAMPLE_BUFFERS: c_uint = 100000;
    pub const SAMPLE_BUFFERS_3DFX: c_uint = 0x8050;
    pub const SAMPLE_BUFFERS_ARB: c_uint = 100000;
    pub const SAMPLE_BUFFERS_BIT_SGIX: c_uint = 0x00000100;
    pub const SAMPLE_BUFFERS_SGIS: c_uint = 100000;
    pub const SAVED: c_uint = 0x8021;
    pub const SAVED_SGIX: c_uint = 0x8021;
    pub const SCREEN: c_uint = 0x800C;
    pub const SCREEN_EXT: c_uint = 0x800C;
    pub const SHARE_CONTEXT_EXT: c_uint = 0x800A;
    pub const SLOW_CONFIG: c_uint = 0x8001;
    pub const SLOW_VISUAL_EXT: c_uint = 0x8001;
    pub const STATIC_COLOR: c_uint = 0x8005;
    pub const STATIC_COLOR_EXT: c_uint = 0x8005;
    pub const STATIC_GRAY: c_uint = 0x8007;
    pub const STATIC_GRAY_EXT: c_uint = 0x8007;
    pub const STENCIL_BUFFER_BIT: c_uint = 0x00000040;
    pub const STENCIL_BUFFER_BIT_SGIX: c_uint = 0x00000040;
    pub const STENCIL_SIZE: c_uint = 13;
    pub const STEREO: c_uint = 6;
    pub const STEREO_NOTIFY_EXT: c_uint = 0x00000000;
    pub const STEREO_NOTIFY_MASK_EXT: c_uint = 0x00000001;
    pub const STEREO_TREE_EXT: c_uint = 0x20F5;
    pub const SWAP_COPY_OML: c_uint = 0x8062;
    pub const SWAP_EXCHANGE_OML: c_uint = 0x8061;
    pub const SWAP_INTERVAL_EXT: c_uint = 0x20F1;
    pub const SWAP_METHOD_OML: c_uint = 0x8060;
    pub const SWAP_UNDEFINED_OML: c_uint = 0x8063;
    pub const SYNC_FRAME_SGIX: c_uint = 0x00000000;
    pub const SYNC_SWAP_SGIX: c_uint = 0x00000001;
    pub const TEXTURE_1D_BIT_EXT: c_uint = 0x00000001;
    pub const TEXTURE_1D_EXT: c_uint = 0x20DB;
    pub const TEXTURE_2D_BIT_EXT: c_uint = 0x00000002;
    pub const TEXTURE_2D_EXT: c_uint = 0x20DC;
    pub const TEXTURE_FORMAT_EXT: c_uint = 0x20D5;
    pub const TEXTURE_FORMAT_NONE_EXT: c_uint = 0x20D8;
    pub const TEXTURE_FORMAT_RGBA_EXT: c_uint = 0x20DA;
    pub const TEXTURE_FORMAT_RGB_EXT: c_uint = 0x20D9;
    pub const TEXTURE_RECTANGLE_BIT_EXT: c_uint = 0x00000004;
    pub const TEXTURE_RECTANGLE_EXT: c_uint = 0x20DD;
    pub const TEXTURE_TARGET_EXT: c_uint = 0x20D6;
    pub const TRANSPARENT_ALPHA_VALUE: c_uint = 0x28;
    pub const TRANSPARENT_ALPHA_VALUE_EXT: c_uint = 0x28;
    pub const TRANSPARENT_BLUE_VALUE: c_uint = 0x27;
    pub const TRANSPARENT_BLUE_VALUE_EXT: c_uint = 0x27;
    pub const TRANSPARENT_GREEN_VALUE: c_uint = 0x26;
    pub const TRANSPARENT_GREEN_VALUE_EXT: c_uint = 0x26;
    pub const TRANSPARENT_INDEX: c_uint = 0x8009;
    pub const TRANSPARENT_INDEX_EXT: c_uint = 0x8009;
    pub const TRANSPARENT_INDEX_VALUE: c_uint = 0x24;
    pub const TRANSPARENT_INDEX_VALUE_EXT: c_uint = 0x24;
    pub const TRANSPARENT_RED_VALUE: c_uint = 0x25;
    pub const TRANSPARENT_RED_VALUE_EXT: c_uint = 0x25;
    pub const TRANSPARENT_RGB: c_uint = 0x8008;
    pub const TRANSPARENT_RGB_EXT: c_uint = 0x8008;
    pub const TRANSPARENT_TYPE: c_uint = 0x23;
    pub const TRANSPARENT_TYPE_EXT: c_uint = 0x23;
    pub const TRUE_COLOR: c_uint = 0x8002;
    pub const TRUE_COLOR_EXT: c_uint = 0x8002;
    pub const UNIQUE_ID_NV: c_uint = 0x20CE;
    pub const USE_GL: c_uint = 1;
    pub const VENDOR: c_uint = 0x1;
    pub const VENDOR_NAMES_EXT: c_uint = 0x20F6;
    pub const VERSION: c_uint = 0x2;
    pub const VIDEO_OUT_ALPHA_NV: c_uint = 0x20C4;
    pub const VIDEO_OUT_COLOR_AND_ALPHA_NV: c_uint = 0x20C6;
    pub const VIDEO_OUT_COLOR_AND_DEPTH_NV: c_uint = 0x20C7;
    pub const VIDEO_OUT_COLOR_NV: c_uint = 0x20C3;
    pub const VIDEO_OUT_DEPTH_NV: c_uint = 0x20C5;
    pub const VIDEO_OUT_FIELD_1_NV: c_uint = 0x20C9;
    pub const VIDEO_OUT_FIELD_2_NV: c_uint = 0x20CA;
    pub const VIDEO_OUT_FRAME_NV: c_uint = 0x20C8;
    pub const VIDEO_OUT_STACKED_FIELDS_1_2_NV: c_uint = 0x20CB;
    pub const VIDEO_OUT_STACKED_FIELDS_2_1_NV: c_uint = 0x20CC;
    pub const VISUAL_CAVEAT_EXT: c_uint = 0x20;
    pub const VISUAL_ID: c_uint = 0x800B;
    pub const VISUAL_ID_EXT: c_uint = 0x800B;
    pub const VISUAL_SELECT_GROUP_SGIX: c_uint = 0x8028;
    pub const WIDTH: c_uint = 0x801D;
    pub const WIDTH_SGIX: c_uint = 0x801D;
    pub const WINDOW: c_uint = 0x8022;
    pub const WINDOW_BIT: c_uint = 0x00000001;
    pub const WINDOW_BIT_SGIX: c_uint = 0x00000001;
    pub const WINDOW_SGIX: c_uint = 0x8022;
    pub const X_RENDERABLE: c_uint = 0x8012;
    pub const X_RENDERABLE_SGIX: c_uint = 0x8012;
    pub const X_VISUAL_TYPE: c_uint = 0x22;
    pub const X_VISUAL_TYPE_EXT: c_uint = 0x22;
    pub const Y_INVERTED_EXT: c_uint = 0x20D4;
    pub const ___GLX_NUMBER_EVENTS: c_uint = 17;
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

    pub struct Glx {
         pub(super) BindChannelToWindowSGIX: FnPtr,
         pub(super) BindHyperpipeSGIX: FnPtr,
         pub(super) BindSwapBarrierNV: FnPtr,
         pub(super) BindSwapBarrierSGIX: FnPtr,
         pub(super) BindTexImageEXT: FnPtr,
         pub(super) BindVideoCaptureDeviceNV: FnPtr,
         pub(super) BindVideoDeviceNV: FnPtr,
         pub(super) BindVideoImageNV: FnPtr,
         pub(super) BlitContextFramebufferAMD: FnPtr,
         pub(super) ChannelRectSGIX: FnPtr,
         pub(super) ChannelRectSyncSGIX: FnPtr,
         pub(super) ChooseFBConfig: FnPtr,
         pub(super) ChooseFBConfigSGIX: FnPtr,
         pub(super) ChooseVisual: FnPtr,
         pub(super) CopyBufferSubDataNV: FnPtr,
         pub(super) CopyContext: FnPtr,
         pub(super) CopyImageSubDataNV: FnPtr,
         pub(super) CopySubBufferMESA: FnPtr,
         pub(super) CreateAssociatedContextAMD: FnPtr,
         pub(super) CreateAssociatedContextAttribsAMD: FnPtr,
         pub(super) CreateContext: FnPtr,
         pub(super) CreateContextAttribsARB: FnPtr,
         pub(super) CreateContextWithConfigSGIX: FnPtr,
         pub(super) CreateGLXPbufferSGIX: FnPtr,
         pub(super) CreateGLXPixmap: FnPtr,
         pub(super) CreateGLXPixmapMESA: FnPtr,
         pub(super) CreateGLXPixmapWithConfigSGIX: FnPtr,
         pub(super) CreateNewContext: FnPtr,
         pub(super) CreatePbuffer: FnPtr,
         pub(super) CreatePixmap: FnPtr,
         pub(super) CreateWindow: FnPtr,
         pub(super) CushionSGI: FnPtr,
         pub(super) DelayBeforeSwapNV: FnPtr,
         pub(super) DeleteAssociatedContextAMD: FnPtr,
         pub(super) DestroyContext: FnPtr,
         pub(super) DestroyGLXPbufferSGIX: FnPtr,
         pub(super) DestroyGLXPixmap: FnPtr,
         pub(super) DestroyHyperpipeConfigSGIX: FnPtr,
         pub(super) DestroyPbuffer: FnPtr,
         pub(super) DestroyPixmap: FnPtr,
         pub(super) DestroyWindow: FnPtr,
         pub(super) EnumerateVideoCaptureDevicesNV: FnPtr,
         pub(super) EnumerateVideoDevicesNV: FnPtr,
         pub(super) FreeContextEXT: FnPtr,
         pub(super) GetAGPOffsetMESA: FnPtr,
         pub(super) GetClientString: FnPtr,
         pub(super) GetConfig: FnPtr,
         pub(super) GetContextGPUIDAMD: FnPtr,
         pub(super) GetContextIDEXT: FnPtr,
         pub(super) GetCurrentAssociatedContextAMD: FnPtr,
         pub(super) GetCurrentContext: FnPtr,
         pub(super) GetCurrentDisplay: FnPtr,
         pub(super) GetCurrentDisplayEXT: FnPtr,
         pub(super) GetCurrentDrawable: FnPtr,
         pub(super) GetCurrentReadDrawable: FnPtr,
         pub(super) GetCurrentReadDrawableSGI: FnPtr,
         pub(super) GetFBConfigAttrib: FnPtr,
         pub(super) GetFBConfigAttribSGIX: FnPtr,
         pub(super) GetFBConfigFromVisualSGIX: FnPtr,
         pub(super) GetFBConfigs: FnPtr,
         pub(super) GetGPUIDsAMD: FnPtr,
         pub(super) GetGPUInfoAMD: FnPtr,
         pub(super) GetMscRateOML: FnPtr,
         pub(super) GetProcAddress: FnPtr,
         pub(super) GetProcAddressARB: FnPtr,
         pub(super) GetSelectedEvent: FnPtr,
         pub(super) GetSelectedEventSGIX: FnPtr,
         pub(super) GetSwapIntervalMESA: FnPtr,
         pub(super) GetSyncValuesOML: FnPtr,
         pub(super) GetTransparentIndexSUN: FnPtr,
         pub(super) GetVideoDeviceNV: FnPtr,
         pub(super) GetVideoInfoNV: FnPtr,
         pub(super) GetVideoSyncSGI: FnPtr,
         pub(super) GetVisualFromFBConfig: FnPtr,
         pub(super) GetVisualFromFBConfigSGIX: FnPtr,
         pub(super) HyperpipeAttribSGIX: FnPtr,
         pub(super) HyperpipeConfigSGIX: FnPtr,
         pub(super) ImportContextEXT: FnPtr,
         pub(super) IsDirect: FnPtr,
         pub(super) JoinSwapGroupNV: FnPtr,
         pub(super) JoinSwapGroupSGIX: FnPtr,
         pub(super) LockVideoCaptureDeviceNV: FnPtr,
         pub(super) MakeAssociatedContextCurrentAMD: FnPtr,
         pub(super) MakeContextCurrent: FnPtr,
         pub(super) MakeCurrent: FnPtr,
         pub(super) MakeCurrentReadSGI: FnPtr,
         pub(super) NamedCopyBufferSubDataNV: FnPtr,
         pub(super) QueryChannelDeltasSGIX: FnPtr,
         pub(super) QueryChannelRectSGIX: FnPtr,
         pub(super) QueryContext: FnPtr,
         pub(super) QueryContextInfoEXT: FnPtr,
         pub(super) QueryCurrentRendererIntegerMESA: FnPtr,
         pub(super) QueryCurrentRendererStringMESA: FnPtr,
         pub(super) QueryDrawable: FnPtr,
         pub(super) QueryExtension: FnPtr,
         pub(super) QueryExtensionsString: FnPtr,
         pub(super) QueryFrameCountNV: FnPtr,
         pub(super) QueryGLXPbufferSGIX: FnPtr,
         pub(super) QueryHyperpipeAttribSGIX: FnPtr,
         pub(super) QueryHyperpipeBestAttribSGIX: FnPtr,
         pub(super) QueryHyperpipeConfigSGIX: FnPtr,
         pub(super) QueryHyperpipeNetworkSGIX: FnPtr,
         pub(super) QueryMaxSwapBarriersSGIX: FnPtr,
         pub(super) QueryMaxSwapGroupsNV: FnPtr,
         pub(super) QueryRendererIntegerMESA: FnPtr,
         pub(super) QueryRendererStringMESA: FnPtr,
         pub(super) QueryServerString: FnPtr,
         pub(super) QuerySwapGroupNV: FnPtr,
         pub(super) QueryVersion: FnPtr,
         pub(super) QueryVideoCaptureDeviceNV: FnPtr,
         pub(super) ReleaseBuffersMESA: FnPtr,
         pub(super) ReleaseTexImageEXT: FnPtr,
         pub(super) ReleaseVideoCaptureDeviceNV: FnPtr,
         pub(super) ReleaseVideoDeviceNV: FnPtr,
         pub(super) ReleaseVideoImageNV: FnPtr,
         pub(super) ResetFrameCountNV: FnPtr,
         pub(super) SelectEvent: FnPtr,
         pub(super) SelectEventSGIX: FnPtr,
         pub(super) SendPbufferToVideoNV: FnPtr,
         pub(super) Set3DfxModeMESA: FnPtr,
         pub(super) SwapBuffers: FnPtr,
         pub(super) SwapBuffersMscOML: FnPtr,
         pub(super) SwapIntervalEXT: FnPtr,
         pub(super) SwapIntervalMESA: FnPtr,
         pub(super) SwapIntervalSGI: FnPtr,
         pub(super) UseXFont: FnPtr,
         pub(super) WaitForMscOML: FnPtr,
         pub(super) WaitForSbcOML: FnPtr,
         pub(super) WaitGL: FnPtr,
         pub(super) WaitVideoSyncSGI: FnPtr,
         pub(super) WaitX: FnPtr,
    }


    impl Glx {

     func!(BindChannelToWindowSGIX, c_int, display: *mut Display, screen: c_int, channel: c_int, window: Window);
     func!(BindHyperpipeSGIX, c_int, dpy: *mut Display, hpId: c_int);
     func!(BindSwapBarrierNV, Bool, dpy: *mut Display, group: GLuint, barrier: GLuint);
     func!(BindSwapBarrierSGIX, (), dpy: *mut Display, drawable: GLXDrawable, barrier: c_int);
     func!(BindTexImageEXT, (), dpy: *mut Display, drawable: GLXDrawable, buffer: c_int, attrib_list: *const c_int);
     func!(BindVideoCaptureDeviceNV, c_int, dpy: *mut Display, video_capture_slot: c_int, device: GLXVideoCaptureDeviceNV);
     func!(BindVideoDeviceNV, c_int, dpy: *mut Display, video_slot: c_int, video_device: c_int, attrib_list: *const c_int);
     func!(BindVideoImageNV, c_int, dpy: *mut Display, VideoDevice: GLXVideoDeviceNV, pbuf: GLXPbuffer, iVideoBuffer: c_int);
     func!(BlitContextFramebufferAMD, (), dstCtx: GLXContext, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);
     func!(ChannelRectSGIX, c_int, display: *mut Display, screen: c_int, channel: c_int, x: c_int, y: c_int, w: c_int, h: c_int);
     func!(ChannelRectSyncSGIX, c_int, display: *mut Display, screen: c_int, channel: c_int, synctype: GLenum);
     func!(ChooseFBConfig, *mut GLXFBConfig, dpy: *mut Display, screen: c_int, attrib_list: *const c_int, nelements: *mut c_int);
     func!(ChooseFBConfigSGIX, *mut GLXFBConfigSGIX, dpy: *mut Display, screen: c_int, attrib_list: *mut c_int, nelements: *mut c_int);
     func!(ChooseVisual, *mut XVisualInfo, dpy: *mut Display, screen: c_int, attribList: *mut c_int);
     func!(CopyBufferSubDataNV, (), dpy: *mut Display, readCtx: GLXContext, writeCtx: GLXContext, readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);
     func!(CopyContext, (), dpy: *mut Display, src: GLXContext, dst: GLXContext, mask: c_long);
     func!(CopyImageSubDataNV, (), dpy: *mut Display, srcCtx: GLXContext, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstCtx: GLXContext, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, width: GLsizei, height: GLsizei, depth: GLsizei);
     func!(CopySubBufferMESA, (), dpy: *mut Display, drawable: GLXDrawable, x: c_int, y: c_int, width: c_int, height: c_int);
     func!(CreateAssociatedContextAMD, GLXContext, id: c_int, share_list: GLXContext);
     func!(CreateAssociatedContextAttribsAMD, GLXContext, id: c_int, share_context: GLXContext, attribList: *const c_int);
     func!(CreateContext, GLXContext, dpy: *mut Display, vis: *mut XVisualInfo, shareList: GLXContext, direct: Bool);
     func!(CreateContextAttribsARB, GLXContext, dpy: *mut Display, config: GLXFBConfig, share_context: GLXContext, direct: Bool, attrib_list: *const c_int);
     func!(CreateContextWithConfigSGIX, GLXContext, dpy: *mut Display, config: GLXFBConfigSGIX, render_type: c_int, share_list: GLXContext, direct: Bool);
     func!(CreateGLXPbufferSGIX, GLXPbufferSGIX, dpy: *mut Display, config: GLXFBConfigSGIX, width: c_int, height: c_int, attrib_list: *mut c_int);
     func!(CreateGLXPixmap, GLXPixmap, dpy: *mut Display, visual: *mut XVisualInfo, pixmap: Pixmap);
     func!(CreateGLXPixmapMESA, GLXPixmap, dpy: *mut Display, visual: *mut XVisualInfo, pixmap: Pixmap, cmap: Colormap);
     func!(CreateGLXPixmapWithConfigSGIX, GLXPixmap, dpy: *mut Display, config: GLXFBConfigSGIX, pixmap: Pixmap);
     func!(CreateNewContext, GLXContext, dpy: *mut Display, config: GLXFBConfig, render_type: c_int, share_list: GLXContext, direct: Bool);
     func!(CreatePbuffer, GLXPbuffer, dpy: *mut Display, config: GLXFBConfig, attrib_list: *const c_int);
     func!(CreatePixmap, GLXPixmap, dpy: *mut Display, config: GLXFBConfig, pixmap: Pixmap, attrib_list: *const c_int);
     func!(CreateWindow, GLXWindow, dpy: *mut Display, config: GLXFBConfig, win: Window, attrib_list: *const c_int);
     func!(CushionSGI, (), dpy: *mut Display, window: Window, cushion: c_float);
     func!(DelayBeforeSwapNV, Bool, dpy: *mut Display, drawable: GLXDrawable, seconds: GLfloat);
     func!(DeleteAssociatedContextAMD, Bool, ctx: GLXContext);
     func!(DestroyContext, (), dpy: *mut Display, ctx: GLXContext);
     func!(DestroyGLXPbufferSGIX, (), dpy: *mut Display, pbuf: GLXPbufferSGIX);
     func!(DestroyGLXPixmap, (), dpy: *mut Display, pixmap: GLXPixmap);
     func!(DestroyHyperpipeConfigSGIX, c_int, dpy: *mut Display, hpId: c_int);
     func!(DestroyPbuffer, (), dpy: *mut Display, pbuf: GLXPbuffer);
     func!(DestroyPixmap, (), dpy: *mut Display, pixmap: GLXPixmap);
     func!(DestroyWindow, (), dpy: *mut Display, win: GLXWindow);
     func!(EnumerateVideoCaptureDevicesNV, *mut GLXVideoCaptureDeviceNV, dpy: *mut Display, screen: c_int, nelements: *mut c_int);
     func!(EnumerateVideoDevicesNV, *mut c_int, dpy: *mut Display, screen: c_int, nelements: *mut c_int);
     func!(FreeContextEXT, (), dpy: *mut Display, context: GLXContext);
     func!(GetAGPOffsetMESA, c_int, pointer: *const c_void);
     func!(GetClientString, *const c_char, dpy: *mut Display, name: c_int);
     func!(GetConfig, c_int, dpy: *mut Display, visual: *mut XVisualInfo, attrib: c_int, value: *mut c_int);
     func!(GetContextGPUIDAMD, c_int, ctx: GLXContext);
     func!(GetContextIDEXT, GLXContextID, context: GLXContext);
     func!(GetCurrentAssociatedContextAMD, GLXContext, );
     func!(GetCurrentContext, GLXContext, );
     func!(GetCurrentDisplay, *mut Display, );
     func!(GetCurrentDisplayEXT, *mut Display, );
     func!(GetCurrentDrawable, GLXDrawable, );
     func!(GetCurrentReadDrawable, GLXDrawable, );
     func!(GetCurrentReadDrawableSGI, GLXDrawable, );
     func!(GetFBConfigAttrib, c_int, dpy: *mut Display, config: GLXFBConfig, attribute: c_int, value: *mut c_int);
     func!(GetFBConfigAttribSGIX, c_int, dpy: *mut Display, config: GLXFBConfigSGIX, attribute: c_int, value: *mut c_int);
     func!(GetFBConfigFromVisualSGIX, GLXFBConfigSGIX, dpy: *mut Display, vis: *mut XVisualInfo);
     func!(GetFBConfigs, *mut GLXFBConfig, dpy: *mut Display, screen: c_int, nelements: *mut c_int);
     func!(GetGPUIDsAMD, c_int, maxCount: c_int, ids: *mut c_int);
     func!(GetGPUInfoAMD, c_int, id: c_int, property: c_int, dataType: GLenum, size: c_int, data: *mut c_void);
     func!(GetMscRateOML, Bool, dpy: *mut Display, drawable: GLXDrawable, numerator: *mut i32, denominator: *mut i32);
     func!(GetProcAddress, __GLXextFuncPtr, procName: *const GLubyte);
     func!(GetProcAddressARB, __GLXextFuncPtr, procName: *const GLubyte);
     func!(GetSelectedEvent, (), dpy: *mut Display, draw: GLXDrawable, event_mask: *mut c_long);
     func!(GetSelectedEventSGIX, (), dpy: *mut Display, drawable: GLXDrawable, mask: *mut c_long);
     func!(GetSwapIntervalMESA, c_int, );
     func!(GetSyncValuesOML, Bool, dpy: *mut Display, drawable: GLXDrawable, ust: *mut i32, msc: *mut i32, sbc: *mut i32);
     func!(GetTransparentIndexSUN, Status, dpy: *mut Display, overlay: Window, underlay: Window, pTransparentIndex: *mut c_long);
     func!(GetVideoDeviceNV, c_int, dpy: *mut Display, screen: c_int, numVideoDevices: c_int, pVideoDevice: *mut GLXVideoDeviceNV);
     func!(GetVideoInfoNV, c_int, dpy: *mut Display, screen: c_int, VideoDevice: GLXVideoDeviceNV, pulCounterOutputPbuffer: *mut c_long, pulCounterOutputVideo: *mut c_long);
     func!(GetVideoSyncSGI, c_int, count: *mut c_int);
     func!(GetVisualFromFBConfig, *mut XVisualInfo, dpy: *mut Display, config: GLXFBConfig);
     func!(GetVisualFromFBConfigSGIX, *mut XVisualInfo, dpy: *mut Display, config: GLXFBConfigSGIX);
     func!(HyperpipeAttribSGIX, c_int, dpy: *mut Display, timeSlice: c_int, attrib: c_int, size: c_int, attribList: *mut c_void);
     func!(HyperpipeConfigSGIX, c_int, dpy: *mut Display, networkId: c_int, npipes: c_int, cfg: *mut GLXHyperpipeConfigSGIX, hpId: *mut c_int);
     func!(ImportContextEXT, GLXContext, dpy: *mut Display, contextID: GLXContextID);
     func!(IsDirect, Bool, dpy: *mut Display, ctx: GLXContext);
     func!(JoinSwapGroupNV, Bool, dpy: *mut Display, drawable: GLXDrawable, group: GLuint);
     func!(JoinSwapGroupSGIX, (), dpy: *mut Display, drawable: GLXDrawable, member: GLXDrawable);
     func!(LockVideoCaptureDeviceNV, (), dpy: *mut Display, device: GLXVideoCaptureDeviceNV);
     func!(MakeAssociatedContextCurrentAMD, Bool, ctx: GLXContext);
     func!(MakeContextCurrent, Bool, dpy: *mut Display, draw: GLXDrawable, read: GLXDrawable, ctx: GLXContext);
     func!(MakeCurrent, Bool, dpy: *mut Display, drawable: GLXDrawable, ctx: GLXContext);
     func!(MakeCurrentReadSGI, Bool, dpy: *mut Display, draw: GLXDrawable, read: GLXDrawable, ctx: GLXContext);
     func!(NamedCopyBufferSubDataNV, (), dpy: *mut Display, readCtx: GLXContext, writeCtx: GLXContext, readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);
     func!(QueryChannelDeltasSGIX, c_int, display: *mut Display, screen: c_int, channel: c_int, x: *mut c_int, y: *mut c_int, w: *mut c_int, h: *mut c_int);
     func!(QueryChannelRectSGIX, c_int, display: *mut Display, screen: c_int, channel: c_int, dx: *mut c_int, dy: *mut c_int, dw: *mut c_int, dh: *mut c_int);
     func!(QueryContext, c_int, dpy: *mut Display, ctx: GLXContext, attribute: c_int, value: *mut c_int);
     func!(QueryContextInfoEXT, c_int, dpy: *mut Display, context: GLXContext, attribute: c_int, value: *mut c_int);
     func!(QueryCurrentRendererIntegerMESA, Bool, attribute: c_int, value: *mut c_int);
     func!(QueryCurrentRendererStringMESA, *const c_char, attribute: c_int);
     func!(QueryDrawable, (), dpy: *mut Display, draw: GLXDrawable, attribute: c_int, value: *mut c_int);
     func!(QueryExtension, Bool, dpy: *mut Display, errorb: *mut c_int, event: *mut c_int);
     func!(QueryExtensionsString, *const c_char, dpy: *mut Display, screen: c_int);
     func!(QueryFrameCountNV, Bool, dpy: *mut Display, screen: c_int, count: *mut GLuint);
     func!(QueryGLXPbufferSGIX, (), dpy: *mut Display, pbuf: GLXPbufferSGIX, attribute: c_int, value: *mut c_int);
     func!(QueryHyperpipeAttribSGIX, c_int, dpy: *mut Display, timeSlice: c_int, attrib: c_int, size: c_int, returnAttribList: *mut c_void);
     func!(QueryHyperpipeBestAttribSGIX, c_int, dpy: *mut Display, timeSlice: c_int, attrib: c_int, size: c_int, attribList: *mut c_void, returnAttribList: *mut c_void);
     func!(QueryHyperpipeConfigSGIX, *mut GLXHyperpipeConfigSGIX, dpy: *mut Display, hpId: c_int, npipes: *mut c_int);
     func!(QueryHyperpipeNetworkSGIX, *mut GLXHyperpipeNetworkSGIX, dpy: *mut Display, npipes: *mut c_int);
     func!(QueryMaxSwapBarriersSGIX, Bool, dpy: *mut Display, screen: c_int, max: *mut c_int);
     func!(QueryMaxSwapGroupsNV, Bool, dpy: *mut Display, screen: c_int, maxGroups: *mut GLuint, maxBarriers: *mut GLuint);
     func!(QueryRendererIntegerMESA, Bool, dpy: *mut Display, screen: c_int, renderer: c_int, attribute: c_int, value: *mut c_int);
     func!(QueryRendererStringMESA, *const c_char, dpy: *mut Display, screen: c_int, renderer: c_int, attribute: c_int);
     func!(QueryServerString, *const c_char, dpy: *mut Display, screen: c_int, name: c_int);
     func!(QuerySwapGroupNV, Bool, dpy: *mut Display, drawable: GLXDrawable, group: *mut GLuint, barrier: *mut GLuint);
     func!(QueryVersion, Bool, dpy: *mut Display, maj: *mut c_int, min: *mut c_int);
     func!(QueryVideoCaptureDeviceNV, c_int, dpy: *mut Display, device: GLXVideoCaptureDeviceNV, attribute: c_int, value: *mut c_int);
     func!(ReleaseBuffersMESA, Bool, dpy: *mut Display, drawable: GLXDrawable);
     func!(ReleaseTexImageEXT, (), dpy: *mut Display, drawable: GLXDrawable, buffer: c_int);
     func!(ReleaseVideoCaptureDeviceNV, (), dpy: *mut Display, device: GLXVideoCaptureDeviceNV);
     func!(ReleaseVideoDeviceNV, c_int, dpy: *mut Display, screen: c_int, VideoDevice: GLXVideoDeviceNV);
     func!(ReleaseVideoImageNV, c_int, dpy: *mut Display, pbuf: GLXPbuffer);
     func!(ResetFrameCountNV, Bool, dpy: *mut Display, screen: c_int);
     func!(SelectEvent, (), dpy: *mut Display, draw: GLXDrawable, event_mask: c_long);
     func!(SelectEventSGIX, (), dpy: *mut Display, drawable: GLXDrawable, mask: c_long);
     func!(SendPbufferToVideoNV, c_int, dpy: *mut Display, pbuf: GLXPbuffer, iBufferType: c_int, pulCounterPbuffer: *mut c_long, bBlock: GLboolean);
     func!(Set3DfxModeMESA, GLboolean, mode: GLint);
     func!(SwapBuffers, (), dpy: *mut Display, drawable: GLXDrawable);
     func!(SwapBuffersMscOML, i32, dpy: *mut Display, drawable: GLXDrawable, target_msc: i32, divisor: i32, remainder: i32);
     func!(SwapIntervalEXT, (), dpy: *mut Display, drawable: GLXDrawable, interval: c_int);
     func!(SwapIntervalMESA, c_int, interval: c_int);
     func!(SwapIntervalSGI, c_int, interval: c_int);
     func!(UseXFont, (), font: Font, first: c_int, count: c_int, list: c_int);
     func!(WaitForMscOML, Bool, dpy: *mut Display, drawable: GLXDrawable, target_msc: i32, divisor: i32, remainder: i32, ust: *mut i32, msc: *mut i32, sbc: *mut i32);
     func!(WaitForSbcOML, Bool, dpy: *mut Display, drawable: GLXDrawable, target_sbc: i32, ust: *mut i32, msc: *mut i32, sbc: *mut i32);
     func!(WaitGL, (), );
     func!(WaitVideoSyncSGI, c_int, divisor: c_int, remainder: c_int, count: *mut c_int);
     func!(WaitX, (), );

    }
}


pub fn load<F>(mut loadfn: F) -> functions::Glx where F: FnMut(&'static str) -> *const c_void {
    #[allow(unused_mut)]
    let mut ctx = Glx {
         BindChannelToWindowSGIX: FnPtr::new(loadfn("glXBindChannelToWindowSGIX")),
         BindHyperpipeSGIX: FnPtr::new(loadfn("glXBindHyperpipeSGIX")),
         BindSwapBarrierNV: FnPtr::new(loadfn("glXBindSwapBarrierNV")),
         BindSwapBarrierSGIX: FnPtr::new(loadfn("glXBindSwapBarrierSGIX")),
         BindTexImageEXT: FnPtr::new(loadfn("glXBindTexImageEXT")),
         BindVideoCaptureDeviceNV: FnPtr::new(loadfn("glXBindVideoCaptureDeviceNV")),
         BindVideoDeviceNV: FnPtr::new(loadfn("glXBindVideoDeviceNV")),
         BindVideoImageNV: FnPtr::new(loadfn("glXBindVideoImageNV")),
         BlitContextFramebufferAMD: FnPtr::new(loadfn("glXBlitContextFramebufferAMD")),
         ChannelRectSGIX: FnPtr::new(loadfn("glXChannelRectSGIX")),
         ChannelRectSyncSGIX: FnPtr::new(loadfn("glXChannelRectSyncSGIX")),
         ChooseFBConfig: FnPtr::new(loadfn("glXChooseFBConfig")),
         ChooseFBConfigSGIX: FnPtr::new(loadfn("glXChooseFBConfigSGIX")),
         ChooseVisual: FnPtr::new(loadfn("glXChooseVisual")),
         CopyBufferSubDataNV: FnPtr::new(loadfn("glXCopyBufferSubDataNV")),
         CopyContext: FnPtr::new(loadfn("glXCopyContext")),
         CopyImageSubDataNV: FnPtr::new(loadfn("glXCopyImageSubDataNV")),
         CopySubBufferMESA: FnPtr::new(loadfn("glXCopySubBufferMESA")),
         CreateAssociatedContextAMD: FnPtr::new(loadfn("glXCreateAssociatedContextAMD")),
         CreateAssociatedContextAttribsAMD: FnPtr::new(loadfn("glXCreateAssociatedContextAttribsAMD")),
         CreateContext: FnPtr::new(loadfn("glXCreateContext")),
         CreateContextAttribsARB: FnPtr::new(loadfn("glXCreateContextAttribsARB")),
         CreateContextWithConfigSGIX: FnPtr::new(loadfn("glXCreateContextWithConfigSGIX")),
         CreateGLXPbufferSGIX: FnPtr::new(loadfn("glXCreateGLXPbufferSGIX")),
         CreateGLXPixmap: FnPtr::new(loadfn("glXCreateGLXPixmap")),
         CreateGLXPixmapMESA: FnPtr::new(loadfn("glXCreateGLXPixmapMESA")),
         CreateGLXPixmapWithConfigSGIX: FnPtr::new(loadfn("glXCreateGLXPixmapWithConfigSGIX")),
         CreateNewContext: FnPtr::new(loadfn("glXCreateNewContext")),
         CreatePbuffer: FnPtr::new(loadfn("glXCreatePbuffer")),
         CreatePixmap: FnPtr::new(loadfn("glXCreatePixmap")),
         CreateWindow: FnPtr::new(loadfn("glXCreateWindow")),
         CushionSGI: FnPtr::new(loadfn("glXCushionSGI")),
         DelayBeforeSwapNV: FnPtr::new(loadfn("glXDelayBeforeSwapNV")),
         DeleteAssociatedContextAMD: FnPtr::new(loadfn("glXDeleteAssociatedContextAMD")),
         DestroyContext: FnPtr::new(loadfn("glXDestroyContext")),
         DestroyGLXPbufferSGIX: FnPtr::new(loadfn("glXDestroyGLXPbufferSGIX")),
         DestroyGLXPixmap: FnPtr::new(loadfn("glXDestroyGLXPixmap")),
         DestroyHyperpipeConfigSGIX: FnPtr::new(loadfn("glXDestroyHyperpipeConfigSGIX")),
         DestroyPbuffer: FnPtr::new(loadfn("glXDestroyPbuffer")),
         DestroyPixmap: FnPtr::new(loadfn("glXDestroyPixmap")),
         DestroyWindow: FnPtr::new(loadfn("glXDestroyWindow")),
         EnumerateVideoCaptureDevicesNV: FnPtr::new(loadfn("glXEnumerateVideoCaptureDevicesNV")),
         EnumerateVideoDevicesNV: FnPtr::new(loadfn("glXEnumerateVideoDevicesNV")),
         FreeContextEXT: FnPtr::new(loadfn("glXFreeContextEXT")),
         GetAGPOffsetMESA: FnPtr::new(loadfn("glXGetAGPOffsetMESA")),
         GetClientString: FnPtr::new(loadfn("glXGetClientString")),
         GetConfig: FnPtr::new(loadfn("glXGetConfig")),
         GetContextGPUIDAMD: FnPtr::new(loadfn("glXGetContextGPUIDAMD")),
         GetContextIDEXT: FnPtr::new(loadfn("glXGetContextIDEXT")),
         GetCurrentAssociatedContextAMD: FnPtr::new(loadfn("glXGetCurrentAssociatedContextAMD")),
         GetCurrentContext: FnPtr::new(loadfn("glXGetCurrentContext")),
         GetCurrentDisplay: FnPtr::new(loadfn("glXGetCurrentDisplay")),
         GetCurrentDisplayEXT: FnPtr::new(loadfn("glXGetCurrentDisplayEXT")),
         GetCurrentDrawable: FnPtr::new(loadfn("glXGetCurrentDrawable")),
         GetCurrentReadDrawable: FnPtr::new(loadfn("glXGetCurrentReadDrawable")),
         GetCurrentReadDrawableSGI: FnPtr::new(loadfn("glXGetCurrentReadDrawableSGI")),
         GetFBConfigAttrib: FnPtr::new(loadfn("glXGetFBConfigAttrib")),
         GetFBConfigAttribSGIX: FnPtr::new(loadfn("glXGetFBConfigAttribSGIX")),
         GetFBConfigFromVisualSGIX: FnPtr::new(loadfn("glXGetFBConfigFromVisualSGIX")),
         GetFBConfigs: FnPtr::new(loadfn("glXGetFBConfigs")),
         GetGPUIDsAMD: FnPtr::new(loadfn("glXGetGPUIDsAMD")),
         GetGPUInfoAMD: FnPtr::new(loadfn("glXGetGPUInfoAMD")),
         GetMscRateOML: FnPtr::new(loadfn("glXGetMscRateOML")),
         GetProcAddress: FnPtr::new(loadfn("glXGetProcAddress")),
         GetProcAddressARB: FnPtr::new(loadfn("glXGetProcAddressARB")),
         GetSelectedEvent: FnPtr::new(loadfn("glXGetSelectedEvent")),
         GetSelectedEventSGIX: FnPtr::new(loadfn("glXGetSelectedEventSGIX")),
         GetSwapIntervalMESA: FnPtr::new(loadfn("glXGetSwapIntervalMESA")),
         GetSyncValuesOML: FnPtr::new(loadfn("glXGetSyncValuesOML")),
         GetTransparentIndexSUN: FnPtr::new(loadfn("glXGetTransparentIndexSUN")),
         GetVideoDeviceNV: FnPtr::new(loadfn("glXGetVideoDeviceNV")),
         GetVideoInfoNV: FnPtr::new(loadfn("glXGetVideoInfoNV")),
         GetVideoSyncSGI: FnPtr::new(loadfn("glXGetVideoSyncSGI")),
         GetVisualFromFBConfig: FnPtr::new(loadfn("glXGetVisualFromFBConfig")),
         GetVisualFromFBConfigSGIX: FnPtr::new(loadfn("glXGetVisualFromFBConfigSGIX")),
         HyperpipeAttribSGIX: FnPtr::new(loadfn("glXHyperpipeAttribSGIX")),
         HyperpipeConfigSGIX: FnPtr::new(loadfn("glXHyperpipeConfigSGIX")),
         ImportContextEXT: FnPtr::new(loadfn("glXImportContextEXT")),
         IsDirect: FnPtr::new(loadfn("glXIsDirect")),
         JoinSwapGroupNV: FnPtr::new(loadfn("glXJoinSwapGroupNV")),
         JoinSwapGroupSGIX: FnPtr::new(loadfn("glXJoinSwapGroupSGIX")),
         LockVideoCaptureDeviceNV: FnPtr::new(loadfn("glXLockVideoCaptureDeviceNV")),
         MakeAssociatedContextCurrentAMD: FnPtr::new(loadfn("glXMakeAssociatedContextCurrentAMD")),
         MakeContextCurrent: FnPtr::new(loadfn("glXMakeContextCurrent")),
         MakeCurrent: FnPtr::new(loadfn("glXMakeCurrent")),
         MakeCurrentReadSGI: FnPtr::new(loadfn("glXMakeCurrentReadSGI")),
         NamedCopyBufferSubDataNV: FnPtr::new(loadfn("glXNamedCopyBufferSubDataNV")),
         QueryChannelDeltasSGIX: FnPtr::new(loadfn("glXQueryChannelDeltasSGIX")),
         QueryChannelRectSGIX: FnPtr::new(loadfn("glXQueryChannelRectSGIX")),
         QueryContext: FnPtr::new(loadfn("glXQueryContext")),
         QueryContextInfoEXT: FnPtr::new(loadfn("glXQueryContextInfoEXT")),
         QueryCurrentRendererIntegerMESA: FnPtr::new(loadfn("glXQueryCurrentRendererIntegerMESA")),
         QueryCurrentRendererStringMESA: FnPtr::new(loadfn("glXQueryCurrentRendererStringMESA")),
         QueryDrawable: FnPtr::new(loadfn("glXQueryDrawable")),
         QueryExtension: FnPtr::new(loadfn("glXQueryExtension")),
         QueryExtensionsString: FnPtr::new(loadfn("glXQueryExtensionsString")),
         QueryFrameCountNV: FnPtr::new(loadfn("glXQueryFrameCountNV")),
         QueryGLXPbufferSGIX: FnPtr::new(loadfn("glXQueryGLXPbufferSGIX")),
         QueryHyperpipeAttribSGIX: FnPtr::new(loadfn("glXQueryHyperpipeAttribSGIX")),
         QueryHyperpipeBestAttribSGIX: FnPtr::new(loadfn("glXQueryHyperpipeBestAttribSGIX")),
         QueryHyperpipeConfigSGIX: FnPtr::new(loadfn("glXQueryHyperpipeConfigSGIX")),
         QueryHyperpipeNetworkSGIX: FnPtr::new(loadfn("glXQueryHyperpipeNetworkSGIX")),
         QueryMaxSwapBarriersSGIX: FnPtr::new(loadfn("glXQueryMaxSwapBarriersSGIX")),
         QueryMaxSwapGroupsNV: FnPtr::new(loadfn("glXQueryMaxSwapGroupsNV")),
         QueryRendererIntegerMESA: FnPtr::new(loadfn("glXQueryRendererIntegerMESA")),
         QueryRendererStringMESA: FnPtr::new(loadfn("glXQueryRendererStringMESA")),
         QueryServerString: FnPtr::new(loadfn("glXQueryServerString")),
         QuerySwapGroupNV: FnPtr::new(loadfn("glXQuerySwapGroupNV")),
         QueryVersion: FnPtr::new(loadfn("glXQueryVersion")),
         QueryVideoCaptureDeviceNV: FnPtr::new(loadfn("glXQueryVideoCaptureDeviceNV")),
         ReleaseBuffersMESA: FnPtr::new(loadfn("glXReleaseBuffersMESA")),
         ReleaseTexImageEXT: FnPtr::new(loadfn("glXReleaseTexImageEXT")),
         ReleaseVideoCaptureDeviceNV: FnPtr::new(loadfn("glXReleaseVideoCaptureDeviceNV")),
         ReleaseVideoDeviceNV: FnPtr::new(loadfn("glXReleaseVideoDeviceNV")),
         ReleaseVideoImageNV: FnPtr::new(loadfn("glXReleaseVideoImageNV")),
         ResetFrameCountNV: FnPtr::new(loadfn("glXResetFrameCountNV")),
         SelectEvent: FnPtr::new(loadfn("glXSelectEvent")),
         SelectEventSGIX: FnPtr::new(loadfn("glXSelectEventSGIX")),
         SendPbufferToVideoNV: FnPtr::new(loadfn("glXSendPbufferToVideoNV")),
         Set3DfxModeMESA: FnPtr::new(loadfn("glXSet3DfxModeMESA")),
         SwapBuffers: FnPtr::new(loadfn("glXSwapBuffers")),
         SwapBuffersMscOML: FnPtr::new(loadfn("glXSwapBuffersMscOML")),
         SwapIntervalEXT: FnPtr::new(loadfn("glXSwapIntervalEXT")),
         SwapIntervalMESA: FnPtr::new(loadfn("glXSwapIntervalMESA")),
         SwapIntervalSGI: FnPtr::new(loadfn("glXSwapIntervalSGI")),
         UseXFont: FnPtr::new(loadfn("glXUseXFont")),
         WaitForMscOML: FnPtr::new(loadfn("glXWaitForMscOML")),
         WaitForSbcOML: FnPtr::new(loadfn("glXWaitForSbcOML")),
         WaitGL: FnPtr::new(loadfn("glXWaitGL")),
         WaitVideoSyncSGI: FnPtr::new(loadfn("glXWaitVideoSyncSGI")),
         WaitX: FnPtr::new(loadfn("glXWaitX")),
    };


     ctx
}


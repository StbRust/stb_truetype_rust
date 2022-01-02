// Generated by Hebron at 1/2/2022 8:34:35 AM

use std;
use c_runtime;
use crate::*;

#[derive(Debug, Copy, Clone)]
pub struct stbrp_context {
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
    pub bottom_y: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct stbrp_node {
    pub x: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct stbrp_rect {
    pub x: i32,
    pub y: i32,
    pub id: i32,
    pub w: i32,
    pub h: i32,
    pub was_packed: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct stbtt_pack_context {
    pub user_allocator_context: *mut u8,
    pub pack_info: *mut u8,
    pub width: i32,
    pub height: i32,
    pub stride_in_bytes: i32,
    pub padding: i32,
    pub skip_missing: i32,
    pub h_oversample: u32,
    pub v_oversample: u32,
    pub pixels: *mut u8,
    pub nodes: *mut u8,
}

#[derive(Debug, Copy, Clone)]
pub struct stbtt_pack_range {
    pub font_size: f32,
    pub first_unicode_codepoint_in_range: i32,
    pub array_of_unicode_codepoints: *mut i32,
    pub num_chars: i32,
    pub chardata_for_range: *mut stbtt_packedchar,
    pub h_oversample: u8,
    pub v_oversample: u8,
}

impl std::default::Default for stbrp_context {
    fn default() -> Self {
        stbrp_context {
            width: 0,
            height: 0,
            x: 0,
            y: 0,
            bottom_y: 0,
        }
    }
}

impl std::default::Default for stbrp_node {
    fn default() -> Self {
        stbrp_node {
            x: 0,
        }
    }
}

impl std::default::Default for stbrp_rect {
    fn default() -> Self {
        stbrp_rect {
            x: 0,
            y: 0,
            id: 0,
            w: 0,
            h: 0,
            was_packed: 0,
        }
    }
}

impl std::default::Default for stbtt_pack_context {
    fn default() -> Self {
        stbtt_pack_context {
            user_allocator_context: std::ptr::null_mut(),
            pack_info: std::ptr::null_mut(),
            width: 0,
            height: 0,
            stride_in_bytes: 0,
            padding: 0,
            skip_missing: 0,
            h_oversample: 0,
            v_oversample: 0,
            pixels: std::ptr::null_mut(),
            nodes: std::ptr::null_mut(),
        }
    }
}

impl std::default::Default for stbtt_pack_range {
    fn default() -> Self {
        stbtt_pack_range {
            font_size: 0,
            first_unicode_codepoint_in_range: 0,
            array_of_unicode_codepoints: std::ptr::null_mut(),
            num_chars: 0,
            chardata_for_range: std::ptr::null_mut(),
            h_oversample: 0,
            v_oversample: 0,
        }
    }
}

pub unsafe fn stbrp_init_target(mut con: *mut stbrp_context, mut pw: i32, mut ph: i32, mut nodes: *mut stbrp_node, mut num_nodes: i32) {
    (*con).width = ((pw) as i32);
    (*con).height = ((ph) as i32);
    (*con).x = ((0) as i32);
    (*con).y = ((0) as i32);
    (*con).bottom_y = ((0) as i32);
}

pub unsafe fn stbrp_pack_rects(mut con: *mut stbrp_context, mut rects: *mut stbrp_rect, mut num_rects: i32) {
    let mut i: i32 = 0;
    i = ((0) as i32);
    ;
    while (i < num_rects) {
        if (*con).x + *rects.offset((i) as isize).w > (*con).width {
            (*con).x = ((0) as i32);
            (*con).y = (((*con).bottom_y) as i32);
        }
        if (*con).y + *rects.offset((i) as isize).h > (*con).height { break; }
        *rects.offset((i) as isize).x = (((*con).x) as i32);
        *rects.offset((i) as isize).y = (((*con).y) as i32);
        *rects.offset((i) as isize).was_packed = ((1) as i32);
        (*con).x += ((*rects.offset((i) as isize).w) as i32);
        if (*con).y + *rects.offset((i) as isize).h > (*con).bottom_y { (*con).bottom_y = (((*con).y + *rects.offset((i) as isize).h) as i32); }
        c_runtime::preInc(&mut i);
    }
    ;
    while (i < num_rects) {
        *rects.offset((i) as isize).was_packed = ((0) as i32);
        c_runtime::preInc(&mut i);
    }
}

pub unsafe fn stbtt_PackBegin(mut spc: *mut stbtt_pack_context, mut pixels: *mut u8, mut pw: i32, mut ph: i32, mut stride_in_bytes: i32, mut padding: i32, mut alloc_context: *mut u8) -> i32 {
    let mut context: *mut stbrp_context = ((c_runtime::malloc(std::mem::size_of::<stbrp_context>() as u64)) as *mut stbrp_context);
    let mut num_nodes: i32 = pw - padding;
    let mut nodes: *mut stbrp_node = ((c_runtime::malloc(std::mem::size_of::<stbrp_node>() as u64 * ((num_nodes) as u64))) as *mut stbrp_node);
    if context == std::ptr::null_mut() || nodes == std::ptr::null_mut() {
        if context != std::ptr::null_mut() { c_runtime::free(((context) as *mut u8)); }
        if nodes != std::ptr::null_mut() { c_runtime::free(((nodes) as *mut u8)); }
        return ((0) as i32);
    }
    (*spc).user_allocator_context = alloc_context;
    (*spc).width = ((pw) as i32);
    (*spc).height = ((ph) as i32);
    (*spc).pixels = pixels;
    (*spc).pack_info = context;
    (*spc).nodes = nodes;
    (*spc).padding = ((padding) as i32);
    (*spc).stride_in_bytes = ((if stride_in_bytes != 0 { stride_in_bytes } else { pw }) as i32);
    (*spc).h_oversample = ((1) as u32);
    (*spc).v_oversample = ((1) as u32);
    (*spc).skip_missing = ((0) as i32);
    stbrp_init_target(context, pw - padding, ph - padding, nodes, num_nodes);
    if (pixels) != std::ptr::null_mut() { c_runtime::memset(pixels, 0, ((pw * ph) as u64)); }
    return ((1) as i32);
}

pub unsafe fn stbtt_PackEnd(mut spc: *mut stbtt_pack_context) {
    c_runtime::free((*spc).nodes);
    c_runtime::free((*spc).pack_info);
}

pub unsafe fn stbtt_PackFontRange(mut spc: *mut stbtt_pack_context, mut fontdata: *mut u8, mut font_index: i32, mut font_size: f32, mut first_unicode_codepoint_in_range: i32, mut num_chars_in_range: i32, mut chardata_for_range: *mut stbtt_packedchar) -> i32 {
    let mut range: stbtt_pack_range = stbtt_pack_range::default();
    range.first_unicode_codepoint_in_range = ((first_unicode_codepoint_in_range) as i32);
    range.array_of_unicode_codepoints = std::ptr::null_mut();
    range.num_chars = ((num_chars_in_range) as i32);
    range.chardata_for_range = chardata_for_range;
    range.font_size = ((font_size) as f32);
    return ((stbtt_PackFontRanges(spc, fontdata, font_index, ((&mut range) as *mut stbtt_pack_range), 1)) as i32);
}

pub unsafe fn stbtt_PackFontRanges(mut spc: *mut stbtt_pack_context, mut fontdata: *mut u8, mut font_index: i32, mut ranges: *mut stbtt_pack_range, mut num_ranges: i32) -> i32 {
    let mut info: stbtt_fontinfo = stbtt_fontinfo::default();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut return_value: i32 = 1;
    let mut rects: *mut stbrp_rect = std::ptr::null_mut();
    i = ((0) as i32);
    ;
    while (i < num_ranges) {
        j = ((0) as i32);
        ;
        while (j < *ranges.offset((i) as isize).num_chars) {
            let hebron_tmp3 = ((0) as u16);
            **ranges.offset((i) as isize).chardata_for_range.offset((j) as isize).x0 = hebron_tmp3;
            **ranges.offset((i) as isize).chardata_for_range.offset((j) as isize).y0 = hebron_tmp3;
            **ranges.offset((i) as isize).chardata_for_range.offset((j) as isize).x1 = hebron_tmp3;
            **ranges.offset((i) as isize).chardata_for_range.offset((j) as isize).y1 = hebron_tmp3;
            c_runtime::preInc(&mut j);
        }
        c_runtime::preInc(&mut i);
    }
    n = ((0) as i32);
    i = ((0) as i32);
    ;
    while (i < num_ranges) {
        n += ((*ranges.offset((i) as isize).num_chars) as i32);
        c_runtime::preInc(&mut i);
    }
    rects = ((c_runtime::malloc(std::mem::size_of::<stbrp_rect>() as u64 * ((n) as u64))) as *mut stbrp_rect);
    if rects == std::ptr::null_mut() { return ((0) as i32); }
    info.userdata = (*spc).user_allocator_context;
    stbtt_InitFont(((&mut info) as *mut stbtt_fontinfo), fontdata, ((stbtt_GetFontOffsetForIndex(fontdata, font_index)) as i32));
    n = ((stbtt_PackFontRangesGatherRects(spc, ((&mut info) as *mut stbtt_fontinfo), ranges, num_ranges, rects)) as i32);
    stbtt_PackFontRangesPackRects(spc, rects, n);
    return_value = ((stbtt_PackFontRangesRenderIntoRects(spc, ((&mut info) as *mut stbtt_fontinfo), ranges, num_ranges, rects)) as i32);
    c_runtime::free(((rects) as *mut u8));
    return ((return_value) as i32);
}

pub unsafe fn stbtt_PackFontRangesPackRects(mut spc: *mut stbtt_pack_context, mut rects: *mut stbrp_rect, mut num_rects: i32) {
    stbrp_pack_rects(((((*spc).pack_info) as *mut stbrp_context) as *mut stbrp_context), rects, num_rects);
}

pub unsafe fn stbtt_PackSetOversampling(mut spc: *mut stbtt_pack_context, mut h_oversample: u32, mut v_oversample: u32) {
    if h_oversample <= ((8) as u32) { (*spc).h_oversample = ((h_oversample) as u32); }
    if v_oversample <= ((8) as u32) { (*spc).v_oversample = ((v_oversample) as u32); }
}

pub unsafe fn stbtt_PackSetSkipMissingCodepoints(mut spc: *mut stbtt_pack_context, mut skip: i32) {
    (*spc).skip_missing = ((skip) as i32);
}

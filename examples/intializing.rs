use std::ffi::c_int;

use freetype::freetype::{FT_Done_FreeType, FT_Init_FreeType, FT_Library, FT_Library_Version};

fn main() {
    let mut ft_library = FT_Library::default();

    unsafe {
        let status = FT_Init_FreeType(&mut ft_library);
        assert_eq!(status, 0);

        let (mut major, mut minor, mut patch): (c_int, c_int, c_int) = Default::default();

        FT_Library_Version(ft_library, &mut major, &mut minor, &mut patch);
        println!("Freetype library version: {}.{}.{}", major, minor, patch);

        let status = FT_Done_FreeType(ft_library);
        assert_eq!(status, 0);
    };
}

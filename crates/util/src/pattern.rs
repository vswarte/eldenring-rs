pub fn find_pattern(input: &str) -> Option<usize> {
    let text_section = broadsword::runtime::get_module_section_range("eldenring.exe", ".text")
        .or_else(|_| broadsword::runtime::get_module_section_range("start_protected_game.exe", ".text"))
        .unwrap();

    let scan_slice = unsafe {
        std::slice::from_raw_parts(
            text_section.start as *const u8,
            text_section.end - text_section.start,
        )
    };

    let pattern = broadsword::scanner::Pattern::from_bit_pattern(input).ok()?;
    let result = broadsword::scanner::threaded::scan(scan_slice, &pattern, None)?;
    Some(text_section.start + result.location)
}

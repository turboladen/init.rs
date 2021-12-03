use overkill_nvim::{
    key_code::KeyCode,
    option::{
        self, flags::AddAssignFlags, BooleanOption, ClipboardSettings, ColorColumnValue,
        CompleteOptSettings, ListCharsSettings, NumberOption, ShortMessItem,
        ShowTablineValue, SignColumnValue, SpellLangValue, StringFlags, StringOption,
    },
    NvimString,
};

#[no_mangle]
pub extern "C" fn init() {
    //-------------------------------------------------------------------------
    // 1. Important
    //-------------------------------------------------------------------------
    option::PasteToggle::set_global(KeyCode::F9).ok();

    //-------------------------------------------------------------------------
    // 2. Moving around, searching and patterns
    //-------------------------------------------------------------------------
    option::SmartCase::set_global(true).ok();
    // 'nosplit' is default as of neovim 0.6.0
    // option::IncCommand::set_global(IncCommandValue::NoSplit).ok();

    //-------------------------------------------------------------------------
    // 4. Displaying text
    //-------------------------------------------------------------------------
    option::BreakIndent::set_global(true).ok();
    option::CmdHeight::set_global(2).ok();
    option::LineBreak::set_global(true).ok();
    option::List::set_global(true).ok();
    option::ListChars::set(
        ListCharsSettings::default()
            .tab2('▸', ' ')
            .trail('·')
            .nbsp('_')
            .extends('❯')
            .precedes('❮'),
    )
    .ok();
    option::Number::set(true).ok();
    option::ScrollOff::set_global(2).ok();
    option::ShowTabline::set_global(ShowTablineValue::Always).ok();

    //-------------------------------------------------------------------------
    // 5. Syntax, highlighting and spelling
    //-------------------------------------------------------------------------
    option::ColorColumn::set(StringFlags::new(vec![
        ColorColumnValue::Absolute(80),
        ColorColumnValue::Absolute(120),
    ]))
    .ok();
    option::CursorLine::set(true).ok();
    option::SpellLang::set(StringFlags::new(vec![SpellLangValue::EnUs])).ok();
    option::Spell::set(true).ok();
    option::SynMaxCol::set_global(250).ok();
    option::TermGuiColors::set_global(true).ok();

    //-------------------------------------------------------------------------
    // 6. Multiple windows
    //-------------------------------------------------------------------------
    option::SplitBelow::set_global(true).ok();
    option::SplitRight::set_global(true).ok();
    // Default in neovim 0.6.0 now.
    // option::Hidden::set_global(true).ok();

    //-------------------------------------------------------------------------
    // 11. Messages and info
    //-------------------------------------------------------------------------
    option::ShortMess::add_assign(ShortMessItem::SuppressInsCompetionMenuMessages).ok();

    //-------------------------------------------------------------------------
    // 12. Selecting text
    //-------------------------------------------------------------------------
    option::Clipboard::set_global(ClipboardSettings::default().unnamed()).ok();

    //-------------------------------------------------------------------------
    // 13. Editing text
    //-------------------------------------------------------------------------
    option::CompleteOpt::set_global(CompleteOptSettings::default().menu().menu_one().no_select())
        .ok();
    option::UndoFile::set_global(true).ok();

    //-------------------------------------------------------------------------
    // 14. Editing text
    //-------------------------------------------------------------------------
    option::ExpandTab::set_global(true).ok();
    option::ShiftWidth::set_global(2).ok();
    option::SmartIndent::set_global(true).ok();
    option::SoftTabStop::set_global(2).ok();
    option::TabStop::set_global(2).ok();

    //-------------------------------------------------------------------------
    // 15. Folding
    //-------------------------------------------------------------------------
    option::FoldEnable::set_global(false).ok();

    //-------------------------------------------------------------------------
    // 18. Reading and writing files
    //-------------------------------------------------------------------------
    option::WriteBackup::set_global(false).ok();

    //-------------------------------------------------------------------------
    // 19. The swap file
    //-------------------------------------------------------------------------
    option::UpdateTime::set_global(500).ok();
    option::SwapFile::set_global(false).ok();

    //-------------------------------------------------------------------------
    // 19. The swap file
    //-------------------------------------------------------------------------
    option::History::set_global(300).ok();

    //-------------------------------------------------------------------------
    // 22. Running make and jumping to errors
    //-------------------------------------------------------------------------
    option::GrepPrg::set_global(NvimString::new_unchecked("rg --vimgrep --files")).ok();

    //-------------------------------------------------------------------------
    // 25. Various
    //-------------------------------------------------------------------------
    option::SignColumn::set(SignColumnValue::Yes).unwrap();
}

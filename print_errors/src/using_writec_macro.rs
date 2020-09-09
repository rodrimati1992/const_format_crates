use const_format::{writec, StrWriter};

fn using_writec(writer: &mut StrWriter) -> const_format::Result {
    // Trying to write to a non-writer
    writec!((), "")?;

    writec!(writer, "{}")?;

    writec!(writer, "{}", foo = "", 100u8)?;

    // trying to write an uninferred integer type
    writec!(writer, "{}", 0)?;

    Ok(())
}
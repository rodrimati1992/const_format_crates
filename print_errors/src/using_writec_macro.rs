use cfmt::{writec, StrWriter};

fn using_writec(writer: &mut StrWriter) -> cfmt::Result {
    // Trying to write to a non-writer
    writec!((), "")?;

    writec!(writer, "{}")?;

    writec!(writer, "{}", foo = "", 100u8)?;

    // trying to write an uninferred integer type
    writec!(writer, "{}", 0)?;

    let a = 0;
    writec!(writer, "{}", a)?;
    writec!(writer, "{b}", b = a)?;
    writec!(writer, "{a}")?;
    writec!(writer, "{a:?}")?;

    Ok(())
}

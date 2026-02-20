mod generic_encode;
use crate::cli::*;
use anyhow::Result;
use generic_encode::file_encode;
use generic_encode::text_encode;

pub fn handle(args: EncodeArgs) -> Result<()> {
    let carrier = args.mask;
    let compression = args.compression;
    let path = args.output;
    // match args.source {
    //     Source::Text { text } => encode_text(carrier, path, text),
    //     Source::File { file } => encode(carrier, path, file),
    // }
    match (&args.text, &args.file) {
        (Some(text), None) => {
            text_encode(carrier, path, text.clone(), compression)?;
            Ok(())
        }
        (None, Some(file)) => {
            file_encode(carrier, path, file.clone(), compression)?;
            Ok(())
        }
        _ => unreachable!(),
    }
}

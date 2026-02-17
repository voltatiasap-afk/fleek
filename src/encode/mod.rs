mod encode_file;
mod encode_text;
use crate::cli::*;
use anyhow::Result;
use encode_file::encode;
use encode_text::encode_text;

pub fn handle(args: EncodeArgs) -> Result<()> {
    let carrier = args.carrier;
    let path = args.output;
    // match args.source {
    //     Source::Text { text } => encode_text(carrier, path, text),
    //     Source::File { file } => encode(carrier, path, file),
    // }
    match (&args.text, &args.file) {
        (Some(text), None) => {
            encode_text(carrier, path, text.clone())?;
            Ok(())
        }
        (None, Some(file)) => {
            encode(carrier, path, file.clone())?;
            Ok(())
        }
        _ => unreachable!(),
    }
}

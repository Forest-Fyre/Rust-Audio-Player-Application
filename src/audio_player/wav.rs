use crate::audio_player::stream_handler::StreamHandler;
use crate::audio_player::byte_parser;

pub fn stream_from_wav_file(file_bytes: &[u8]) -> StreamHandler {
    //I will need to actually read the header data here at some point to ensure the data is in the right format
    
    let samples = byte_parser::to_type_little_endian::<i16>(&file_bytes[44..]);
    
    StreamHandler::from_samples(samples)
}

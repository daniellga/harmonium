test_that(
  "hfile works.",
  {
    harmonium_path = system.file(package = "harmonium")
    filepath = file.path(harmonium_path, "testfiles", "gs-16b-2c-44100hz.flac")
    expect_equal(HFile$params(filepath), c(44100.00000,698194.00000,2.00000,15.8320635))
    expect_equal(HFile$verify(filepath), "passed")
    expect_equal(HFile$metadata(filepath, HMetadataType$text), list(c(tag_key = "title", tag_std_key = "TrackTitle", tag_value = "Galway"
    ), c(tag_key = "artist", tag_std_key = "Artist", tag_value = "Kevin MacLeod"
    ), c(tag_key = "encoder", tag_std_key = "Encoder", tag_value = "Lavf56.40.101"
    )))
    expect_equal(HFile$metadata(filepath, HMetadataType$visual), list())

    # wav file having "\0" character which is not supported by R.
    filepath2 = file.path(harmonium_path, "testfiles", "gs-16b-1c-44100hz.wav")
    expect_error(HFile$metadata(filepath2, HMetadataType$text))
    
    # Decode and stream tests.
    dtype = HDataType$float32
    l = HFile$decode(filepath, dtype)
    expect_equal(l[[1]]$shape(), c(2, 698194))
    expect_equal(l[[2]], 44100L)
    
    decoder_stream = HFile$decode_stream(filepath, 1000L, dtype)
    harray = decoder_stream$stream()
    expect_equal(harray$shape(), c(2, 1000))
    
    for(i in 1:698) {
      decoder_stream$stream()
    }
    
    null = decoder_stream$stream()
    expect_null(null)
    
    
  }
)

test_that(
  "hfile works.",
  {
    # The CI may not have audio support available.
    skip_on_ci()

    harmonium_path = system.file(package = "harmonium")
    filepath = file.path(harmonium_path, "testfiles", "gs-16b-2c-44100hz.flac")
    expect_equal(HFile$params(filepath), c(44100.00000,698194.00000,2.00000,15.8320635))
    expect_equal(HFile$verify(filepath), "passed")
    expect_equal(HFile$metadata(filepath, HMetadataType$Text), list(c(tag_key = "title", tag_std_key = "TrackTitle", tag_value = "Galway"
    ), c(tag_key = "artist", tag_std_key = "Artist", tag_value = "Kevin MacLeod"
    ), c(tag_key = "encoder", tag_std_key = "Encoder", tag_value = "Lavf56.40.101"
    )))
    expect_equal(HFile$metadata(filepath, HMetadataType$Visual), list())

    # wav file having "\0" character which is not supported by R.
    filepath2 = file.path(harmonium_path, "testfiles", "gs-16b-1c-44100hz.wav")
    expect_error(HFile$metadata(filepath2, HMetadataType$Text))
    
    # Decode and stream tests.
    dtype = HDataType$Float32
    l = HFile$decode(filepath, dtype)
    expect_equal(l$harray()$shape(), c(2, 698194))
    expect_equal(l$sr(), 44100L)
    
    decoder_stream = HFile$decode_stream(filepath, 1000L, dtype)
    harray = decoder_stream$stream()
    expect_equal(harray$shape(), c(2, 1000))
    
    for(i in 1:697) {
      decoder_stream$stream()
    }
    
    expect_error(decoder_stream$stream())
  }
)

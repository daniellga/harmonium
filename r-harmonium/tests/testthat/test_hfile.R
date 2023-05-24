test_that(
  "hfile works.",
  {
    filepath = file.path("..", "..", "..", "testfiles", "gs-16b-2c-44100hz.flac")
    expect_equal(HFile$get_params_from_file(filepath), c(44100.00000,698194.00000,2.00000,15.8320635))
    expect_equal(HFile$verify_file(filepath), "passed")
    expect_equal(HFile$metadata_from_file(filepath, HMetadataType$text), list(c(tag_key = "title", tag_std_key = "TrackTitle", tag_value = "Galway"
    ), c(tag_key = "artist", tag_std_key = "Artist", tag_value = "Kevin MacLeod"
    ), c(tag_key = "encoder", tag_std_key = "Encoder", tag_value = "Lavf56.40.101"
    )))
    expect_equal(HFile$metadata_from_file("../../../testfiles/gs-16b-2c-44100hz.flac", HMetadataType$visual), list())

    # wav file having "\0" character which is not supported by R.
    expect_error(HFile$metadata_from_file("../../../testfiles/gs-16b-1c-44100hz.wav", HMetadataType$text))
  }
)

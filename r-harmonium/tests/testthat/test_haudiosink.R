test_that(
  "haudiosink works.",
  {
    filepath = file.path("..", "..", "..", "testfiles", "gs-16b-2c-44100hz.wav")
    
    # haudiosink from haudio.
    haudiosink = HAudioSink$new()
    expect_true(haudiosink$is_empty())
    haudio = HAudio$new_from_file(filepath, dtype = HDataType$float32)
    expect_silent(haudiosink$append_from_haudio(haudio))
    expect_false(haudiosink$is_empty())

    # haudiosink from file.
    haudiosink = HAudioSink$new()
    expect_true(haudiosink$is_empty())
    expect_equal(haudiosink$len(), 0)
    haudiosink$append_from_file(filepath)
    expect_false(haudiosink$is_empty())
    haudiosink$append_from_file(filepath)
    expect_equal(haudiosink$len(), 2)
    #haudiosink$skip_one()
    #haudiosink$len() == 1
    haudiosink$set_speed(2)
    haudiosink$set_volume(2)
    expect_equal(haudiosink$speed(), 2)
    expect_equal(haudiosink$volume(), 2)
    haudiosink$pause()
    expect_true(haudiosink$is_paused())
    haudiosink$play()
    haudiosink$stop()
    expect_false(haudiosink$is_paused())
    haudiosink$append_from_file(filepath)
    haudiosink$append_from_file(filepath)
    haudiosink$play()
    expect_equal(haudiosink$len(), 2)
    haudiosink$clear()
    expect_equal(haudiosink$len(), 0)
    expect_true(haudiosink$is_paused())
    haudiosink$append_from_file(filepath)
    expect_equal(haudiosink$len(), 1)

    # needed so no song is played when test is run.
    rm(haudiosink)
    gc()

    # haudiosink audio_configs.
    expect_true(class(HAudioSink$audio_supported_configs()) == "character")
    expect_true(class(HAudioSink$audio_default_device()) == "character")
    expect_true(class(HAudioSink$audio_output_devices()) == "character")
  }
)

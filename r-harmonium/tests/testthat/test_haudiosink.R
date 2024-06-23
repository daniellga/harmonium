# The CI may not have audio support available.
skip_on_ci()

test_that(
  "haudiosink works.",
  {
    harmonium_path = system.file(package = "harmonium")
    filepath = file.path(harmonium_path, "testfiles", "gs-16b-2c-44100hz.wav")
    
    haudiosink_from_harray = function() {
      haudiosink = HAudioSink$new()
      expect_true(haudiosink$is_empty())
      l = HFile$decode(filepath, dtype = HDataType$Float32)
      expect_silent(haudiosink$append_from_harray(l$harray(), l$sr()))
      expect_false(haudiosink$is_empty())
    }

    haudiosink_from_file = function() {
      haudiosink = HAudioSink$new()
      expect_true(haudiosink$is_empty())
      expect_equal(haudiosink$len(), 0)
      haudiosink$append_from_file(filepath)
      expect_no_error(haudiosink$try_seek(4))
      expect_false(haudiosink$is_empty())
      haudiosink$append_from_file(filepath)
      expect_equal(haudiosink$len(), 2)
      # skip_one test is not modifying the len properly. It seems to work when running outside testthat. https://github.com/RustAudio/rodio/issues/497
      #haudiosink$skip_one()
      #expect_equal(haudiosink$len(), 1)
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
    }

    haudiosink_from_harray()
    haudiosink_from_file()

    # haudiosink audio_configs.
    expect_true(class(HAudioSink$audio_supported_configs()) == "character")
    expect_true(class(HAudioSink$audio_default_device()) == "character")
    expect_true(class(HAudioSink$audio_output_devices()) == "character")
  }
)

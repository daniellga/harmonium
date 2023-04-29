# resample test

# test example from rubato repo.
data = matrix(0, nrow = 1024, ncol = 2)
haudio = HAudio$new_from_values(data, 44100, dtype = HDataType$float64())
haudio$resample_sincfixedin(
  48000,
  2,
  256,
  0.95,
  256,
  "linear",
  "blackmanharris2",
  1024
)
expect_equal(haudio$len(), 1948)
expect_equal(haudio$sr(), 48000)

## prove/schnorrkel

[![PDF of Slope](pdf_small.svg)](pdf.svg)[![Regression](regression_small.svg)](regression.svg)

#### Additional Statistics:

Lower boundEstimateUpper boundSlope99.349 µs99.491 µs99.630 µsR²0.99546170.99574640.9954745Mean99.473 µs99.584 µs99.690 µsStd. Dev.509.86 ns554.61 ns589.67 nsMedian99.523 µs99.826 µs99.970 µsMAD293.07 ns499.68 ns822.83 ns

#### Additional Plots:

- [Typical](typical.svg)
- [Mean](mean.svg)
- [Std. Dev.](SD.svg)
- [Median](median.svg)
- [MAD](MAD.svg)
- [Slope](slope.svg)

#### Understanding this report:

The plot on the left displays the average time per iteration for this benchmark. The shaded region
shows the estimated probability of an iteration taking a certain amount of time, while the line
shows the mean. Click on the plot for a larger view showing the outliers.

The plot on the right shows the linear regression calculated from the measurements. Each point
represents a sample, though here it shows the total time for the sample rather than time per
iteration. The line is the line of best fit for these measurements.

See [the\
documentation](https://bheisler.github.io/criterion.rs/book/user_guide/command_line_output.md#additional-statistics) for more details on the additional statistics.

This report was generated by
[Criterion.rs](https://github.com/bheisler/criterion.rs), a statistics-driven benchmarking
library in Rust.

# Histogram simulator

Tweak histogram settings for data fitting purposes.

<div id="custom-data-modal-overlay">
    <div id="custom-data-modal">
        <p>Please, insert your custom data below as a JSON array.</p>
        <textarea rows="30"></textarea>
    </div>
</div>
<div id="histogram-simulator">
    <div id="histogram-props">
        <h3>Histogram properties</h3>
        <div class="input-group">
            <label for="kind">Histogram kind <span class="required">*</span></label>
            <select name="kind" id="kind">
                <option value="exponential" selectedT>Exponential</option>
                <option value="linear">Linear</option>
            </select>
        </div>
        <div class="input-group">
            <label for="lower-bound">Lower bound</label>
            <input name="lower-bound" id="lower-bound" type="number" value="1" />
        </div>
        <div class="input-group">
            <label for="upper-bound">Upper bound <span class="required">*</span></label>
            <input name="upper-bound" id="upper-bound" type="number" value="500" />
        </div>
        <div class="input-group">
            <label for="bucket-count">Bucket count <span class="required">*</span> </label>
            <input name="bucket-count" id="bucket-count" type="number" value="20" />
        </div>
    </div>
    <div id="data-options">
        <h3>Data options <span class="required">*</span></h3>
        <div class="input-group">
            <label for="normally-distributed">Generate normally distributed data</label>
            <input name="data-options" value="normally-distributed" id="normally-distributed" type="radio" />
        </div>
        <div class="input-group">
            <label for="log-normally-distributed">Generate log-normally distributed data</label>
            <input name="data-options" value="log-normally-distributed" id="log-normally-distributed" type="radio" checked />
        </div>
        <div class="input-group">
            <label for="uniformly-distributed">Generate uniformly distributed data</label>
            <input name="data-options" value="uniformly-distributed" id="uniformly-distributed" type="radio" />
        </div>
        <div class="input-group" id="custom-data-input-group">
            <label for="custom">Use custom data</label>
            <input name="data-options" value="custom" id="custom" type="radio" />
        </div>
    </div>
    <div id="histogram-chart"></div>
    <p id="histogram-chart-legend"><p>
</div>

### Observations

The `lowerBound` parameter is is always optional. If omitted, this parameter defaults to 1.

The leftmost bucket is the **underflow bucket**, used for values at the bottom of the expected range - generally, this means between 0 and 1.

The rightmost bucket is the overflow bucket, used for values at or above `upperBound` - all values greater or equal to `upperBound` go here.

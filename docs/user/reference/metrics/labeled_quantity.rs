/* ANCHOR: all */

// ANCHOR: import
import org.mozilla.yourApplication.GleanMetrics.Gfx
// ANCHOR_END: import

// ANCHOR: set
Gfx.display["width"].set(width)
Gfx.display["heigth"].set(height)
// ANCHOR_END: set

// ANCHOR: testGetValue
assertEquals(433, Gfx.display["width"].testGetValue())
assertEquals(42, Gfx.display["height"].testGetValue())
// ANCHOR_END: testGetValue

// ANCHOR: testGetNumRecordedErrors
assertEquals(
    0,
    Gfx.display["width"].testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
)
assertEquals(
    42,
    Gfx.display["height"].testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
)
// ANCHOR_END: testGetNumRecordedErrors

/* ANCHOR_END: all */

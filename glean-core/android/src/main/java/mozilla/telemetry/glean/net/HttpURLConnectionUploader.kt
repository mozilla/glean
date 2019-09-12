/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.net

import android.util.Log
import androidx.annotation.VisibleForTesting
import java.io.IOException
import java.net.CookieHandler
import java.net.CookieManager
import java.net.HttpURLConnection
import java.net.MalformedURLException
import java.net.URL

/**
 * A simple ping Uploader, which implements a "send once" policy, never
 * storing or attempting to send the ping again.
 */
class HttpURLConnectionUploader : PingUploader {
    companion object {
        private const val LOG_TAG: String = "glean/HttpConnUploader"
        // The timeout, in milliseconds, to use when connecting to the server.
        const val DEFAULT_CONNECTION_TIMEOUT = 10000
        // The timeout, in milliseconds, to use when reading from the server.
        const val DEFAULT_READ_TIMEOUT = 30000
    }

    /**
     * Synchronously upload a ping to a server.
     *
     * @param url the URL path to upload the data to
     * @param data the serialized text data to send
     * @param headers a [HeadersList] containing String to String [Pair] with
     *        the first entry being the header name and the second its value.
     *
     * @return true if the ping was correctly dealt with (sent successfully
     *         or faced an unrecoverable error), false if there was a recoverable
     *         error callers can deal with.
     */
    @Suppress("ReturnCount", "MagicNumber")
    override fun upload(url: String, data: String, headers: HeadersList): Boolean {
        var connection: HttpURLConnection? = null
        try {
            connection = openConnection(url)
            connection.requestMethod = "POST"
            connection.connectTimeout = DEFAULT_CONNECTION_TIMEOUT
            connection.readTimeout = DEFAULT_READ_TIMEOUT
            connection.doOutput = true

            headers.forEach { (headerName, headerValue) ->
                connection.setRequestProperty(headerName, headerValue)
            }

            // Make sure we are not sending cookies. Unfortunately, HttpURLConnection doesn't
            // offer a better API to do that, so we nuke all cookies going to our telemetry
            // endpoint.
            removeCookies(url)

            // Finally upload.
            val responseCode = doUpload(connection, data)

            Log.d(LOG_TAG, "Ping upload: $responseCode")

            when (responseCode) {
                in HttpURLConnection.HTTP_OK..(HttpURLConnection.HTTP_OK + 99) -> {
                    // Known success errors (2xx):
                    // 200 - OK. Request accepted into the pipeline.

                    // We treat all success codes as successful upload even though we only expect 200.
                    Log.d(LOG_TAG, "Ping successfully sent ($responseCode)")
                    return true
                }
                in HttpURLConnection.HTTP_BAD_REQUEST..(HttpURLConnection.HTTP_BAD_REQUEST + 99) -> {
                    // Known client (4xx) errors:
                    // 404 - not found - POST/PUT to an unknown namespace
                    // 405 - wrong request type (anything other than POST/PUT)
                    // 411 - missing content-length header
                    // 413 - request body too large (Note that if we have badly-behaved clients that
                    //       retry on 4XX, we should send back 202 on body/path too long).
                    // 414 - request path too long (See above)

                    // Something our client did is not correct. It's unlikely that the client is going
                    // to recover from this by re-trying again, so we just log and error and report a
                    // successful upload to the service.
                    Log.e(LOG_TAG, "Server returned client error code: $responseCode")
                    return true
                }
                else -> {
                    // Known other errors:
                    // 500 - internal error

                    // For all other errors we log a warning an try again at a later time.
                    Log.w(LOG_TAG, "Server returned response code: $responseCode")
                    return false
                }
            }
        } catch (e: MalformedURLException) {
            // There's nothing we can do to recover from this here. So let's just log an error and
            // notify the service that this job has been completed - even though we didn't upload
            // anything to the server.
            Log.e(LOG_TAG, "Could not upload telemetry due to malformed URL", e)
            return true
        } catch (e: IOException) {
            Log.w(LOG_TAG, "IOException while uploading ping", e)
            return false
        } finally {
            connection?.disconnect()
        }
    }

    /**
     * Remove all the cookies related to the server endpoint, so
     * that nothing other than ping data travels to the endpoint.
     *
     * @param submissionUrl the submissionUrl, including server address and path
     */
    internal fun removeCookies(submissionUrl: String) {
        (CookieHandler.getDefault() as? CookieManager)?.let { cookieManager ->
            val url = try {
                val fullUrl = URL(submissionUrl)
                // We just need the protocol, host and port.
                val onlyHostUrl = fullUrl.protocol + "://" + fullUrl.host + ":" + fullUrl.port
                URL(onlyHostUrl)
            } catch (e: MalformedURLException) {
                null
            }

            url?.let {
                val uri = it.toURI()
                for (cookie in cookieManager.cookieStore.get(uri)) {
                    cookieManager.cookieStore.remove(uri, cookie)
                }
            }
        }
    }

    @Throws(IOException::class)
    fun doUpload(connection: HttpURLConnection, data: String): Int {
        connection.outputStream.bufferedWriter().use {
            it.write(data)
            it.flush()
        }

        return connection.responseCode
    }

    @VisibleForTesting @Throws(IOException::class)
    fun openConnection(url: String): HttpURLConnection {
        return URL(url).openConnection() as HttpURLConnection
    }
}

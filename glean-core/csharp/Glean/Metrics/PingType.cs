

using Mozilla.Glean.FFI;
using System;

namespace Mozilla.Glean.Private
{
    /// <summary>
    ///  An enum with no values for convenient use as the default set of reason codes.
    /// </summary>
    public enum NoReasonCodes
    {
    }

    internal class PingTypeBase
    {
        internal string name;
        internal UInt64 handle = 0;
    }

    public sealed class PingType<NoReasonCodes>
    {
        private readonly string name;
        private readonly bool includeClientId;
        private readonly bool sendIfEmpty;
        private readonly LibGleanFFI.PingTypeHandle handle;

        public PingType (
            string name,
            bool includeClientId,
            bool sendIfEmpty,
            string[] reasonCodes
            ) : this(new LibGleanFFI.PingTypeHandle(), name, includeClientId, sendIfEmpty)
        {
            if (reasonCodes == null)
            {
                reasonCodes = new string[] { };
            }

            handle = LibGleanFFI.glean_new_ping_type(
                name: name,
                include_client_id: includeClientId == true ? (byte)1 : (byte)0,
                send_if_empty: sendIfEmpty == true ? (byte)1 : (byte)0,
                reason: reasonCodes,
                reason_codes_len: reasonCodes.Length
            );
            // TODO: RegisterPingType
            // Glean.RegisterPingType(this);
        }

        internal PingType(
            LibGleanFFI.PingTypeHandle handle,
            string name,
            bool includeClientId,
            bool sendIfEmpty
            )
        {
            this.name = name;
            this.handle = handle;
            this.includeClientId = includeClientId;
            this.sendIfEmpty = sendIfEmpty;
        }

        /// <summary>
        ///  Collect and submit the ping for eventual upload.
        ///  
        ///  While the collection of metrics into pings happens synchronously, the
        ///  ping queuing and ping uploading happens asyncronously.
        ///  There are no guarantees that this will happen immediately.
        ///  
        ///  If the ping currently contains no content, it will not be queued.
        /// </summary>
        public void Submit()
        {
            // TODO: submit pings
            //Glean.SubmitPings(this)
        }

    }
}

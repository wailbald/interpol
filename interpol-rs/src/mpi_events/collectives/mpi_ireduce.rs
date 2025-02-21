use crate::types::{MpiComm, MpiOp, MpiRank, MpiReq, Tsc};
use crate::{impl_builder_error, impl_register};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// A structure that stores information about `MPI_Ireduce` calls.
///
/// The information stored are:
/// - the rank of the process making the call to `MPI_Ireduce`;
/// - the rank of the root process making the broadcast;
/// - the number of bytes exchanged;
/// - the type of MPI reduction operation;
/// - the identifier of the MPI communicator;
/// - the identifier of the MPI request;
/// - the tag of the communication;
/// - the current value of the Time Stamp counter before the call to `MPI_Ireduce`.
/// - the duration of the call.
#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpiIreduce {
    current_rank: MpiRank,
    partner_rank: MpiRank,
    nb_bytes: u32,
    op_type: MpiOp,
    comm: MpiComm,
    req: MpiReq,
    tsc: Tsc,
    duration: Tsc,
}

impl MpiIreduce {
    /// Creates a new `MpiIreduce` structure from the specified parameters.
    pub fn new(
        current_rank: MpiRank,
        partner_rank: MpiRank,
        nb_bytes: u32,
        op_type: MpiOp,
        comm: MpiComm,
        req: MpiReq,
        tsc: Tsc,
        duration: Tsc,
    ) -> Self {
        MpiIreduce {
            current_rank,
            partner_rank,
            nb_bytes,
            op_type,
            comm,
            req,
            tsc,
            duration,
        }
    }
}

impl_builder_error!(MpiIreduceBuilderError);
impl_register!(MpiIreduce);

#[cfg(test)]
mod tests {
    use super::*;
    const MPI_COMM_WORLD: i32 = 0;

    #[test]
    fn builds() {
        let ireduce_new = MpiIreduce::new(0, 1, 8, MpiOp::Sum, MPI_COMM_WORLD, 7, 1024, 2048);
        let ireduce_builder = MpiIreduceBuilder::default()
            .current_rank(0)
            .partner_rank(1)
            .nb_bytes(8)
            .op_type(MpiOp::Sum)
            .comm(MPI_COMM_WORLD)
            .req(7)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiIreduce`");

        assert_eq!(ireduce_new, ireduce_builder);
    }

    #[test]
    fn serializes() {
        let ireduce = MpiIreduce::new(0, 0, 8, MpiOp::Prod, MPI_COMM_WORLD, 7, 1024, 2048);
        let json = String::from("{\"current_rank\":0,\"partner_rank\":0,\"nb_bytes\":8,\"op_type\":\"Prod\",\"comm\":0,\"req\":7,\"tsc\":1024,\"duration\":2048}");
        let serialized = serde_json::to_string(&ireduce).expect("failed to serialize `MpiIreduce`");

        assert_eq!(json, serialized);
    }

    #[test]
    fn deserializes() {
        let ireduce = MpiIreduceBuilder::default()
            .current_rank(1)
            .partner_rank(0)
            .nb_bytes(8)
            .op_type(MpiOp::Max)
            .comm(MPI_COMM_WORLD)
            .req(7)
            .tsc(1024)
            .duration(2048)
            .build()
            .expect("failed to build `MpiIreduce`");
        let serialized =
            serde_json::to_string_pretty(&ireduce).expect("failed to serialize `MpiIreduce`");
        let deserialized: MpiIreduce =
            serde_json::from_str(&serialized).expect("failed to deserialize `MpiIreduce`");

        assert_eq!(ireduce, deserialized);
    }
}

// Wrap in a new error
error_chain!{
    foreign_links {
        Grpc(::grpcio::Error);
    }
}

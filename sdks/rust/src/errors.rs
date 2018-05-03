error_chain!{
    foreign_links {
        Grpc(::grpcio::Error);
    }
}

pub trait IRepoMapper<A, D> {
    fn do_to_aggregate(object_do: D) -> A;

    fn aggregate_to_do(arggregate: A) -> D;
}
pub struct SuperDspData<const len: usize, T> {
    arr: [T; len]
}

pub struct Pipeline<const len: usize>{
    // Theadpool to run the pipeline
    threadpool: rayon::ThreadPool,

}
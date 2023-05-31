use super::*;

pub struct ProcStatmINode(ProcessRef);

impl ProcStatmINode {
    pub fn new(process_ref: &ProcessRef) -> Arc<dyn INode> {
        Arc::new(File::new(Self(Arc::clone(process_ref))))
    }
}

impl ProcINode for ProcStatmINode {
    fn generate_data_in_bytes(&self) -> vfs::Result<Vec<u8>> {
        // size       (1) total program size
        //             (same as VmSize in /proc/[pid]/status)
        // resident   (2) resident set size
        //             (inaccurate; same as VmRSS in /proc/[pid]/status)
        // shared     (3) number of resident shared pages
        //             (i.e., backed by a file)
        //             (inaccurate; same as RssFile+RssShmem in
        //             /proc/[pid]/status)
        // text       (4) text (code)
        // lib        (5) library (unused since Linux 2.6; always 0)
        // data       (6) data + stack
        // dt         (7) dirty pages (unused since Linux 2.6; always 0)
        // 1467549 169344 136320 67949 0 609534 0
        let size = 1467549;
        let resident = 169344;
        let shared = 136320;
        let text = 67949;
        let lib = 0;
        let data = 609534;
        let dt = 0;

        // Put the information together in the specific format
        let result = format!(
            "{} \
            {} \
            {} \
            {} \
            {} \
            {} \
            {}",
            size, resident, shared, text, lib, data, dt
        )
        .into_bytes();
        Ok(result)
    }
}

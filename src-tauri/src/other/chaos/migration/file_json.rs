

use super::*;

struct FileJson<D:MigData>{
    now_version:VersionMarkType,
    data:D
}


impl<D:MigData> Mig<D> for FileJson<D>{

}

impl<D:MigData> MigVersion for FileJson<D> {
    fn now_version()->VersionMarkType {
        "123"
    }
}
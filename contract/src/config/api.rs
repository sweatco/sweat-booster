use sweat_booster_model::api::ConfigApi;

use crate::Contract;

impl ConfigApi for Contract {
    fn set_base_uri(&mut self, base_uri: String) {
        self.assert_oracle();

        let mut metadata = self.metadata.get().expect("No metadata found");
        metadata.base_uri = Some(base_uri);
        self.metadata.replace(&metadata);
    }
}
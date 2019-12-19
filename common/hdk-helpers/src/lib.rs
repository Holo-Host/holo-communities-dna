use hdk::prelude::*;
use std::collections::HashSet;

pub fn commit_if_not_in_chain(entry: &Entry) -> ZomeApiResult<Address> {
    // use query to check the chain. When there is an HDK function doing this directly use it instead
    let existing_entries = hdk::query(entry.entry_type().into(), 0, 0)?;
    if existing_entries.contains(&entry.address()) {
        // do nothing and be happy
        Ok(entry.address())
    } else {
        // do the commit as usual
        hdk::commit_entry(entry)
    }
}

pub trait DagList<E: Into<JsonString> + Clone> {
    fn author(
        &mut self,
        table: &str,
        content: E,
        prev_authored: Option<Address>,
        prev_foreign: Option<Address>,
    ) -> ZomeApiResult<Address>;

    fn author_root_address(&self) -> Address;

    fn foreign_root_address(&self, table: &str) -> Address;

    fn most_recent_authored(&self, table: &str) -> ZomeApiResult<Option<Address>>;

    fn get_prev_authored(&self, address: &Address) -> ZomeApiResult<Option<Address>>;

    fn get_prev_foreign(&self, address: &Address) -> ZomeApiResult<Option<Address>>;

    fn get_next(&self, table: &str, address: &Address) -> ZomeApiResult<Vec<Address>>;

    fn add_content_dag(
        &mut self,
        table: &str,
        content: E,
        fallback_root: &Address,
    ) -> ZomeApiResult<Address> {
        // get the most recent address of entry this agent authored (or some starting point)
        let most_recent_authored = self
            .most_recent_authored(table)?
            .unwrap_or(self.author_root_address());
        // get the entries after this one all the way to the tip (or some starting point)
        let most_recent_foreign = self
            .get_content_dag(table, &self.foreign_root_address(table), None, None)?
            .0
            .last()
            .cloned()
            .unwrap_or(fallback_root.clone());
        self.author(
            table,
            content,
            Some(most_recent_authored),
            Some(most_recent_foreign),
        )
    }

    fn get_content_dag(
        &self,
        table: &str,
        since: &Address,
        limit: Option<usize>,
        _backsteps: Option<usize>,
    ) -> ZomeApiResult<(Vec<Address>, bool)> {
        // step back to find some suitable starting entries (skip for now and just use current)
        let current = since;

        // traverse the unknown graph and store the result
        // uses non-recursive DFS topological sort
        // as described here https://sergebg.blogspot.com/2014/11/non-recursive-dfs-topological-sort.html
        let mut to_visit = vec![(current.clone(), false)];
        let mut visited = HashSet::<Address>::new();
        let mut sort_stack = vec![];
        let mut more = false;

        while let Some((current, postprocess)) = to_visit.pop() {
            if postprocess {
                sort_stack.push(current.clone());
            } else {
                // push a second time but with post_process=true
                to_visit.push((current.clone(), true));
                for next in self.get_next(table, &current)? {
                    if !visited.contains(&next) {
                        if limit.is_none() || visited.len() < limit.unwrap() {
                            to_visit.push((next.clone(), false));
                            visited.insert(next.clone());
                        } else {
                            more = true;
                        }
                    }
                }
            }
        }
        sort_stack.pop(); // don't include the 'since' hash
        sort_stack.reverse();
        Ok((sort_stack, more))
    }
}

pub trait DagListDebug<E: Into<JsonString> + Clone>: DagList<E> {
    fn adjacency_list(
        &self,
        table: &str,
        root: &Address,
    ) -> ZomeApiResult<Vec<(Address, Address)>> {
        // just use a regular DFS here
        let current = root;
        // traverse the unknown graph and store the edges
        let mut to_visit = vec![current.clone()];
        let mut visited = HashSet::<Address>::new();
        let mut edges = HashSet::<(Address, Address)>::new();
        while let Some(current) = to_visit.pop() {
            for next in self.get_next(table, &current)? {
                edges.insert((current.clone(), next.clone()));
                if !visited.contains(&next) {
                    to_visit.push(next.clone());
                    visited.insert(next.clone());
                }
            }
        }
        Ok(edges.into_iter().collect())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::iter::FromIterator;

    // create a test mock graph store

    struct TestStore {
        entry_store: HashMap<Address, JsonString>,
        forward_link_store: HashMap<Address, Vec<Address>>,
        prev_authored_link_store: HashMap<Address, Address>,
        prev_foreign_link_store: HashMap<Address, Address>,
        author_list: Vec<Address>,
    }

    impl TestStore {
        fn new() -> Self {
            Self {
                entry_store: HashMap::new(),
                forward_link_store: HashMap::new(),
                prev_authored_link_store: HashMap::new(),
                prev_foreign_link_store: HashMap::new(),
                author_list: Vec::new(),
            }
        }
    }

    impl DagList<i32> for TestStore {
        fn author(
            &mut self,
            _table: &str,
            content: i32,
            prev_authored: Option<Address>,
            prev_foreign: Option<Address>,
        ) -> ZomeApiResult<Address> {
            let entry_address = Address::from(format!("{}", content));
            // add the new entry
            self.entry_store
                .insert(entry_address.clone(), content.into());
            self.forward_link_store
                .insert(entry_address.clone(), Vec::new());
            // add the links from and to previous entries
            if let Some(prev_authored) = prev_authored {
                if self.forward_link_store.get(&prev_authored).is_none() {
                    self.forward_link_store
                        .insert(prev_authored.clone(), Vec::new());
                }
                self.forward_link_store
                    .get_mut(&prev_authored)
                    .unwrap()
                    .push(entry_address.clone());
                self.prev_authored_link_store
                    .insert(entry_address.clone(), prev_authored);
            }
            if let Some(prev_foreign) = prev_foreign {
                if self.forward_link_store.get(&prev_foreign).is_none() {
                    self.forward_link_store
                        .insert(prev_foreign.clone(), Vec::new());
                }
                self.forward_link_store
                    .get_mut(&prev_foreign)
                    .unwrap()
                    .push(entry_address.clone());
                self.prev_foreign_link_store
                    .insert(entry_address.clone(), prev_foreign);
            }
            // add to the author list
            self.author_list.push(entry_address.clone());
            Ok(entry_address)
        }

        fn author_root_address(&self) -> Address {
            Address::from("agent_root")
        }

        fn foreign_root_address(&self, _table: &str) -> Address {
            Address::from("foreign_root")
        }

        fn get_prev_authored(&self, address: &Address) -> ZomeApiResult<Option<Address>> {
            Ok(self.prev_authored_link_store.get(address).cloned())
        }

        fn get_prev_foreign(&self, address: &Address) -> ZomeApiResult<Option<Address>> {
            Ok(self.prev_foreign_link_store.get(address).cloned())
        }

        fn most_recent_authored(&self, _table: &str) -> ZomeApiResult<Option<Address>> {
            Ok(self.author_list.last().cloned())
        }

        fn get_next(&self, _table: &str, address: &Address) -> ZomeApiResult<Vec<Address>> {
            Ok(self
                .forward_link_store
                .get(address)
                .unwrap_or(&Vec::new())
                .to_vec())
        }
    }

    impl DagListDebug<i32> for TestStore {}

    #[test]
    fn test_get_nothing() {
        let mut store = TestStore::new();
        let root_addr = store.author("test_table", -1, None, None).unwrap();
        // This retrieves everything
        assert_eq!(
            store.get_content_dag("test_table", &root_addr, None, None),
            Ok((vec![], false)),
        );
    }

    #[test]
    fn test_get_singleton() {
        // 0
        let mut store = TestStore::new();
        let addr = store.author("test_table", 0, None, None).unwrap();
        // This retrieves everything
        assert_eq!(
            store.get_content_dag("test_table", &addr, None, None),
            Ok((vec![], false)),
        );
        assert_eq!(store.adjacency_list("test_table", &addr), Ok(vec![]));
    }

    #[test]
    fn test_get_2_chain() {
        // 0->1
        let mut store = TestStore::new();
        let root_addr = store.author("test_table", 0, None, None).unwrap();
        let tip_addr = store
            .author("test_table", 1, Some(root_addr.clone()), None)
            .unwrap();

        // This retrieves everything if started at the root
        assert_eq!(
            store.get_content_dag("test_table", &root_addr, None, None),
            Ok((vec![tip_addr.clone()], false)),
        );
        // starting at the tip gets nothing
        assert_eq!(
            store.get_content_dag("test_table", &tip_addr, None, None),
            Ok((vec![], false)),
        );

        assert_eq!(
            store.adjacency_list("test_table", &root_addr),
            Ok(vec![(root_addr, tip_addr)])
        );
    }

    #[test]
    fn test_get_fork() {
        // 0->1
        //  \>2
        let mut store = TestStore::new();
        let root_addr = store.author("test_table", 0, None, None).unwrap();
        let tip1_addr = store
            .author("test_table", 1, Some(root_addr.clone()), None)
            .unwrap();
        let tip2_addr = store
            .author("test_table", 2, None, Some(root_addr.clone()))
            .unwrap();

        // This retrieves everything if started at the root
        assert_eq!(
            store.get_content_dag("test_table", &root_addr, None, None),
            Ok((vec![tip1_addr.clone(), tip2_addr.clone()], false)),
        );

        assert_eq!(
            store.adjacency_list("test_table", &root_addr).map(|v| v.len()).unwrap_or(0),
            2
        );

        /*
         * TODO: the order of these results seems to be non-deterministic
        assert_eq!(
            store.adjacency_list("test_table", &root_addr),
            Ok(vec![(root_addr.clone(), tip1_addr), (root_addr, tip2_addr)])
        );
        */
    }

    #[test]
    fn test_two_authors() {
        // 0->1->2->3
        //  \     \   \
        //   \     \   \
        //    \>10->11->12
        //
        let mut store = TestStore::new();
        let addr0 = store.author("test_table", 0, None, None).unwrap();
        let addr1 = store
            .author("test_table", 1, Some(addr0.clone()), None)
            .unwrap();
        let addr2 = store
            .author("test_table", 2, Some(addr1.clone()), None)
            .unwrap();
        let addr3 = store
            .author("test_table", 3, Some(addr2.clone()), None)
            .unwrap();

        let addr10 = store
            .author("test_table", 10, None, Some(addr0.clone()))
            .unwrap();
        let addr11 = store
            .author("test_table", 11, Some(addr10.clone()), Some(addr2.clone()))
            .unwrap();
        let addr12 = store
            .author("test_table", 12, Some(addr11.clone()), Some(addr3.clone()))
            .unwrap();

        // This retrieves everything if started at the root
        assert_eq!(
            store
                .get_content_dag("test_table", &addr0, None, None)
                .unwrap()
                .0,
            vec![
                addr1.clone(),
                addr2.clone(),
                addr3.clone(),
                addr10.clone(),
                addr11.clone(),
                addr12.clone()
            ],
        );

        // This retrieves only things after 2 if started at 2
        assert_eq!(
            store
                .get_content_dag("test_table", &addr2, None, None)
                .unwrap()
                .0,
            vec![addr3.clone(), addr11.clone(), addr12.clone()],
        );

        // The limit can be used to truncate
        assert_eq!(
            store.get_content_dag("test_table", &addr0, Some(3), None),
            Ok((vec![addr1.clone(), addr10.clone(), addr11.clone()], true)),
        );

        assert_eq!(
            HashSet::<(Address, Address)>::from_iter(
                store
                    .adjacency_list("test_table", &addr0)
                    .unwrap()
                    .into_iter()
            ),
            HashSet::from_iter(
                vec![
                    (addr0.clone(), addr1.clone()),
                    (addr1.clone(), addr2.clone()),
                    (addr2.clone(), addr3.clone()),
                    (addr0.clone(), addr10.clone()),
                    (addr10.clone(), addr11.clone()),
                    (addr11.clone(), addr12.clone()),
                    (addr2.clone(), addr11.clone()),
                    (addr3.clone(), addr12.clone())
                ]
                .into_iter()
            )
        );
    }

    #[test]
    fn test_add_single_content_dag() {
        let mut store = TestStore::new();
        let root_addr = store.author("test_table", -1, None, None).unwrap();
        let addr = store.add_content_dag("test_table", 0, &root_addr).unwrap();
        assert_eq!(
            store.get_content_dag("test_table", &root_addr, None, None),
            Ok((vec![addr], false)),
        );
    }

    #[test]
    fn test_add_chain_content_dag() {
        let mut store = TestStore::new();
        let root_addr = store.author("test_table", -1, None, None).unwrap();
        let addr0 = store.add_content_dag("test_table", 0, &root_addr).unwrap();
        let addr1 = store.add_content_dag("test_table", 1, &root_addr).unwrap();
        let addr2 = store.add_content_dag("test_table", 2, &root_addr).unwrap();
        let addr3 = store.add_content_dag("test_table", 3, &root_addr).unwrap();
        let addr4 = store.add_content_dag("test_table", 4, &root_addr).unwrap();
        let addr5 = store.add_content_dag("test_table", 5, &root_addr).unwrap();

        // can get the lot
        assert_eq!(
            store.get_content_dag("test_table", &root_addr, None, None),
            Ok((
                vec![
                    addr0.clone(),
                    addr1.clone(),
                    addr2.clone(),
                    addr3.clone(),
                    addr4.clone(),
                    addr5.clone()
                ],
                false
            )),
        );

        // can limit starting at the root
        assert_eq!(
            store.get_content_dag("test_table", &root_addr, Some(3), None),
            Ok((vec![addr0.clone(), addr1.clone(), addr2.clone()], true)),
        );

        // can limit starting at a 'since'
        assert_eq!(
            store.get_content_dag("test_table", &addr1, Some(3), None),
            Ok((vec![addr2.clone(), addr3.clone(), addr4.clone()], true)),
        );
    }
}

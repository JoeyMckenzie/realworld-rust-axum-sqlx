#[derive(Clone)]
pub struct PaginationQueryBuilder {
    params: ArticlePaginationQueryParams,
}

impl PaginationQueryBuilder {
    pub fn new(url: String) -> Self {
        Self {
            params: ArticlePaginationQueryParams {
                url,
                limit: None,
                offset: None,
                author: None,
                tag: None,
                favorited: None,
            },
        }
    }

    pub fn with_limit(&mut self, limit: usize) -> Self {
        self.params.limit = Some(limit);
        self.clone()
    }

    pub fn with_offset(&mut self, offset: usize) -> Self {
        self.params.offset = Some(offset);
        self.clone()
    }

    pub fn with_author(&mut self, author: String) -> Self {
        self.params.author = Some(author);
        self.clone()
    }

    pub fn with_tag(&mut self, tag: String) -> Self {
        self.params.tag = Some(tag);
        self.clone()
    }

    pub fn with_favorited(&mut self, favorited: String) -> Self {
        self.params.favorited = Some(favorited);
        self.clone()
    }

    pub fn build(self) -> ArticlePaginationQueryParams {
        self.params
    }
}

#[derive(Clone)]
pub struct ArticlePaginationQueryParams {
    url: String,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub author: Option<String>,
    pub tag: Option<String>,
    pub favorited: Option<String>,
}

impl ArticlePaginationQueryParams {
    pub fn to_query_string(&self) -> String {
        let mut query_string = format!("{}?", self.url);
        let mut param_count = 0;

        if let Some(limit) = self.limit {
            param_count += 1;
            query_string.push_str(&format!("limit={}&", limit));
        }

        if let Some(offset) = self.offset {
            param_count += 1;
            query_string.push_str(&format!("offset={}&", offset));
        }

        if let Some(author) = &self.author {
            // `let` bindings with additional conditions are unstable for now, so we add a nested `if` here
            if !author.is_empty() {
                param_count += 1;
                query_string.push_str(&format!("author={}&", *author));
            }
        }

        if let Some(tag) = &self.tag {
            // `let` bindings with additional conditions are unstable for now, so we add a nested `if` here
            if !tag.is_empty() {
                param_count += 1;
                query_string.push_str(&format!("tag={}&", *tag));
            }
        }

        if let Some(favorited) = &self.favorited {
            // `let` bindings with additional conditions are unstable for now, so we add a nested `if` here
            if !favorited.is_empty() {
                param_count += 1;
                query_string.push_str(&format!("favorited={}", *favorited));
            }
        }

        if param_count == 0 {
            // in the case we don't have any query params, remove the '?' from the url
            query_string.pop();
        }

        if query_string.ends_with('&') {
            query_string.pop();
        }

        query_string
    }
}

#[cfg(test)]
mod article_pagination_query_params_should {
    use super::*;

    #[test]
    fn should_properly_build_a_full_param_struct() {
        // arrange
        let limit = 20_usize;
        let offset = 0_usize;
        let author = "the_beercoder".to_string();
        let tag = "rust".to_string();
        let favorited = "another_user".to_string();

        // act
        let params = PaginationQueryBuilder::new("https://reddit.com/r/rust".to_string())
            .with_limit(limit)
            .with_offset(offset)
            .with_author(author.clone())
            .with_tag(tag.clone())
            .with_favorited(favorited.clone())
            .build();

        // assert
        assert!(params.limit.is_some());
        assert!(params.offset.is_some());
        assert!(params.author.is_some());
        assert!(params.favorited.is_some());
        assert!(params.tag.is_some());
        assert_eq!(params.limit.unwrap(), limit);
        assert_eq!(params.offset.unwrap(), offset);
        assert_eq!(params.author.unwrap(), author);
        assert_eq!(params.tag.unwrap(), tag);
        assert_eq!(params.favorited.unwrap(), favorited);
    }

    #[test]
    fn should_properly_build_a_mut_param_struct_with_conditions() {
        // arrange
        let limit = 20_usize;
        let offset = 0_usize;
        let author = "the_beercoder".to_string();
        let tag = "rust".to_string();
        let favorited = "another_user".to_string();

        // act
        let mut params_builder = PaginationQueryBuilder::new("https://reddit.com/r/rust".to_string())
            .with_limit(limit)
            .with_offset(offset);

        if true {
            params_builder.with_author(author.clone());
        }

        if false {
            params_builder.with_tag(tag);
        }

        if true {
            params_builder.with_favorited(favorited.clone());
        }

        let params = params_builder.build();

        // assert
        assert!(params.limit.is_some());
        assert!(params.offset.is_some());
        assert!(params.author.is_some());
        assert!(params.favorited.is_some());
        assert!(params.tag.is_none());
        assert_eq!(params.limit.unwrap(), limit);
        assert_eq!(params.offset.unwrap(), offset);
        assert_eq!(params.author.unwrap(), author);
        assert_eq!(params.favorited.unwrap(), favorited);
    }

    #[test]
    fn should_properly_build_a_partial_param_struct() {
        // arrange
        let limit = 20_usize;
        let offset = 0_usize;
        let author = "the_beercoder".to_string();

        // act
        let params = PaginationQueryBuilder::new("https://reddit.com/r/rust".to_string())
            .with_limit(limit)
            .with_offset(offset)
            .with_author(author.clone())
            .build();

        // assert
        assert!(params.limit.is_some());
        assert!(params.offset.is_some());
        assert!(params.author.is_some());
        assert!(params.favorited.is_none());
        assert!(params.tag.is_none());
        assert_eq!(params.limit.unwrap(), limit);
        assert_eq!(params.offset.unwrap(), offset);
        assert_eq!(params.author.unwrap(), author);
    }

    #[test]
    fn should_not_include_empty_string_params() {
        // arrange
        let limit = 20_usize;
        let offset = 0_usize;
        let tag = String::default();
        let favorited = "another_user".to_string();
        let expected_url = "https://reddit.com/r/rust?limit=20&offset=0&favorited=another_user";

        // act
        let params = PaginationQueryBuilder::new("https://reddit.com/r/rust".to_string())
            .with_limit(limit)
            .with_offset(offset)
            .with_favorited(favorited)
            .with_tag(tag)
            .build();

        let query_string = params.to_query_string();

        // assert
        assert_eq!(expected_url.to_string(), query_string);
    }

    #[test]
    fn should_properly_build_query_string_with_all_params() {
        // arrange
        let limit = 20_usize;
        let offset = 0_usize;
        let author = "the_beercoder".to_string();
        let tag = "rust".to_string();
        let favorited = "another_user".to_string();
        let expected_url =
            "https://reddit.com/r/rust?limit=20&offset=0&author=the_beercoder&tag=rust&favorited=another_user";

        // act
        let params = PaginationQueryBuilder::new("https://reddit.com/r/rust".to_string())
            .with_limit(limit)
            .with_offset(offset)
            .with_author(author)
            .with_tag(tag)
            .with_favorited(favorited)
            .build();

        let query_string = params.to_query_string();

        // assert
        assert_eq!(expected_url.to_string(), query_string);
    }

    #[test]
    fn should_properly_build_query_string_with_partial_params() {
        // arrange
        let limit = 20_usize;
        let author = "the_beercoder".to_string();
        let expected_url = "https://reddit.com/r/rust?limit=20&author=the_beercoder";

        // act
        let params = PaginationQueryBuilder::new("https://reddit.com/r/rust".to_string())
            .with_limit(limit)
            .with_author(author)
            .build();

        let query_string = params.to_query_string();

        // assert
        assert_eq!(expected_url.to_string(), query_string);
    }
}

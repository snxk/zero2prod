//! test/server.rs

#[cfg(test)]
async fn test_server() {
  let mut res = surf::get("127.0.0.1:3000").await?;
 assert_eq!(res.status(), 200);

}
use actix_web::{test, web, App};
use zero2prod::health_check;

#[actix_rt::test]
async fn test_health_check() {
    let mut app =
        test::init_service(App::new().route("/health_check", web::get().to(health_check))).await;

    let req = test::TestRequest::get().uri("/health_check").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), 200);
}

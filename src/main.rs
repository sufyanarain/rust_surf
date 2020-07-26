// use async_std::task;
// use surf;

// async fn fetching(url:&str) -> Result<String, surf::Exception>{
//     surf::get(url).recv_string().await

    
// }

// async fn execute(){
//     match fetching("https://portal.piaic.org/").await{
//         Ok(response)=> println!("{}",response),
//         Err(error) => println!("{}",error),
//     }
// }

// // async fn execute2(){
// //     match fetching("https://github.com/").await{
// //         Ok(response)=> println!("{}",response),
// //         Err(error) => println!("{}",error),
// //     }
// // }

// fn main(){
    
//     task::block_on(execute());
//     // task::block_on(execute2());
// }


use async_std::task;
use surf;
use futures::try_join;


fn main() -> Result<(),Box<dyn std::error::Error + Send + Sync>> {
    task::block_on(
        async{
            let req1 = surf::get("https://docs.rs/surf/1.0.1/surf/struct.Client.html").recv_string();
            let req2 = surf::get("https://docs.rs/surf/1.0.1/surf/struct.Client.html").recv_string();
            let (response_01,response_02) = futures::future::try_join(req1,req2).await?;
            dbg!("{:?}",response_01);
            dbg!("{:?}",response_02);
            Ok(())
        }
    )

}



















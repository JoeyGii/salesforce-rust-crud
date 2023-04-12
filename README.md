# Steps 

 Add a .env file to the root of the project and create the below variables:

- username=(your salesforce username)
- pw=(your salesforce pw and security token)
- client_id=(client id of a connected app)
- client_secret(secret of connected app)
- uri=(your uri e.g example--example.sandbox.my.salesforce.com)

run the following command: cargo run {sObject} {id} {field api} {value}

example: cargo run Account 001D400000jC6woIAC New_Id__c 1234

# extra help 

- install rust https://www.rust-lang.org/tools/install
- create a salesforce connected app https://help.salesforce.com/s/articleView?id=sf.connected_app_overview.htm&type=5
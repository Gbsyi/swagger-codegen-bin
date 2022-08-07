# Swagger Api Codegen (V3)
Generates api code from online swagger generator. For now, it's only for third version of generator  
# Usage
First you must set generator options in codegen.config file.  
 - api_url - url to your swagger api json file. Example: http://localhost:8080/swagger/swagger.json
 - gen_type - Type of generator. Available values: ["Client", "Server", "Config", "Documentation"]
 - lang - generator language. Available languages can be gained from here: [Client](https://generator3.swagger.io/api/types?types=client&version=V3) | [Server](https://generator3.swagger.io/api/types?types=server&version=V3) | [Config](https://generator3.swagger.io/api/types?types=config&version=V3) | [Documentation](https://generator3.swagger.io/api/types?types=documentation&version=V3). 
 - folder - output folder where generator should generate code.  
<span style="color:orange">WARNING: Generator deletes all files in output folder before copying</span> 

using Microsoft.AspNetCore.Mvc;

namespace dotnet_test.Controllers;

[ApiController]
[Route("[controller]")]
public class HelloWorldController : ControllerBase
{
    [HttpGet]
    public string Get() {
        return "Hello, World!";
    }
}
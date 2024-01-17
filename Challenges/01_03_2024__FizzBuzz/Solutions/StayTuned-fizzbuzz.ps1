$fizzBuzz = foreach ($i in (1..101)){

    if ($i % 3 -eq 0){ 
        Write-Host "Fizz"
    } elseif ($i % 5 -eq 0) {
        Write-host "Buzz"
    } else {
        Write-host $i
    }
}
$fizzBuzz

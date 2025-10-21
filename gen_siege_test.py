import random

def generate_random_ip():
    """Generate a random IP address"""
    return f"{random.randint(1, 255)}.{random.randint(0, 255)}.{random.randint(0, 255)}.{random.randint(1, 255)}"

def generate_siege_urls(num_ips=10000, output_file="urls.txt"):
    """
    Generate Siege URLs file with POST requests containing different IP addresses
    
    Args:
        num_ips: Number of unique IP addresses to generate
        output_file: Output filename for Siege URLs file
    """
    with open(output_file, 'w') as f:
        for i in range(num_ips):
            ip = generate_random_ip()
            # Siege format: URL POST key=value
            # For JSON body, we use the format with Content-Type header
            f.write(f'http://localhost:5000/logs POST [{{"ip": "{ip}"}}] H:Content-Type: application/json\n')
    
    print(f"Generated {num_ips} URLs in {output_file}")
    print(f"\nTo run with Siege:")
    print(f"  siege -f {output_file} -c 10 -r 1")
    print(f"\nOptions explanation:")
    print(f"  -f {output_file}  : Use the URLs file")
    print(f"  -c 10            : 10 concurrent users")
    print(f"  -r 1             : 1 repetition (each URL hit once)")

if __name__ == "__main__":
    generate_siege_urls()


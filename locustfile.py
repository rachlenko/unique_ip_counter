
from locust import HttpUser, task, between
from faker import Faker
from datetime import datetime, timezone

# Create a Faker instance to generate fake data
fake = Faker()

class WebsiteUser(HttpUser):
    host = "http://localhost:5000"
    wait_time = between(1, 5)  # Wait time between tasks in seconds

    @task
    def send_log(self):
        # Generate a random IPv4 address
        ip_address = fake.ipv4()
        timestamp = datetime.now(timezone.utc).isoformat()
        url = "/test"
        # Define the JSON payload
        # "{\"timestamp\": \"$timestamp\", \"ip\": \"$ip\", \"url\": \"/test\"}"
        payload = {"timestamp": timestamp, "ip": ip_address, "url": url}

        # Send the POST request
        self.client.post("/logs", json=payload)

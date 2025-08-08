# Security Policy

## Our Commitment

Security is important even in colorimetry libraries. We take security vulnerabilities seriously and are committed to
addressing them promptly and transparently.

## What Constitutes a Security Vulnerability

A security vulnerability is an issue that could:

- Allow unauthorized access to sensitive data processed by dir_spec
- Enable data manipulation or corruption in color calculations
- Permit code injection through malicious color data
- Bypass input validation mechanisms
- Enable denial of service attacks through resource exhaustion
- Expose sensitive information in error messages or logs
- Allow arbitrary file access through color profile loading

**Not security vulnerabilities:**

- General bugs that don't compromise security
- Feature requests or enhancements
- Performance issues
- Documentation errors
- Mathematical precision differences in color calculations

## Reporting a Vulnerability

If you discover a security issue, please bring it to our attention right away!

### Reporting Process

Please **DO NOT** file a public issue. Instead, report security vulnerabilities through
[GitHub's private vulnerability reporting feature](https://github.com/aaronmallen/dir_spec/security/advisories/new).

Your report should include:

- Description of the vulnerability
- Steps to reproduce the issue
- Potential impact of the vulnerability
- Affected versions (if known)
- Suggested fix (if any)
- Your contact information for follow-up questions

### What to Expect

After you've submitted your report:

1. **Acknowledgment** - You'll receive confirmation within 24 hours
2. **Investigation** - We'll investigate and keep you updated on our findings
3. **Resolution** - Once we've determined the impact and developed a fix:

- We'll patch the vulnerability
- We'll coordinate disclosure timing with you
- We'll make an announcement to the community if warranted
- You'll be credited for the discovery (unless you prefer to remain anonymous)

### Response Timeline

- **24 hours** - Initial response acknowledging receipt
- **72 hours** - Preliminary assessment of impact and severity
- **7 days** - Detailed investigation results and remediation plan
- **30 days** - Target for patch release (may vary based on complexity)

## Disclosure Policy

We follow responsible disclosure practices:

1. **Confirm** the problem and determine affected versions
2. **Audit** code to find any similar problems
3. **Prepare** fixes for all supported versions
4. **Coordinate** with the reporter on disclosure timing
5. **Release** patches as soon as possible
6. **Publish** a security advisory with appropriate details

## Supported Versions

| Version | Support |
|:-------:| :-----: |
|  0.0.1  |   ✅    |

### Key

| Symbol |    Meaning     |
| :----: | :------------: |
|   ✅   |   Supported    |
|   ❌   | Not Supported  |
|   🧪   |  Experimental  |
|   🚧   | In Development |

## Security Best Practices

When contributing to dir_spec, please follow these security guidelines:

- Never commit sensitive data (API keys, personal information)
- Validate and sanitize all color inputs
- Use safe parsing methods for color data formats
- Avoid executing arbitrary code from color profile data
- Handle file I/O operations securely when loading color profiles
- Keep dependencies up to date
- Follow the principle of least privilege
- Use secure defaults in configuration

## Comments on this Policy

If you have suggestions on how this process could be improved, please submit a pull request or open an issue for
discussion.

## Contact

For urgent security matters that require immediate attention, you can also reach out to the maintainers directly
through GitHub.

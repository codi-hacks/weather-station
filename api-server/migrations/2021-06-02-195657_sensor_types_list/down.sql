DELETE FROM sensor_types
WHERE id IN (
    '6a3afc2e-f38e-4044-972d-49388f51053a', 
    'cb5bfac7-e839-4cea-898c-5db3a95fa638',
    '012e0074-7b09-4b65-a3e0-bb27b9e934dd',
    '46a4fb66-4671-4d06-b531-1cad0c8629c5',
    '5332e7ca-2bfc-423e-a333-711f993bc3ab',
    'aaed6b1d-75b9-4130-929d-df16421f01f0')
RETURNING *;
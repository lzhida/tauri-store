<script lang="ts">
  import { Link } from '$components/link';
  import { Changelog } from '$components/container';

  const version = '0.10.3';

  const url = 'https://www.npmjs.com/package/@tauri-store/shared';
</script>

<Changelog.Version {version} />

<Changelog.Dependencies {version}>
  <ul>
    <li>Update <Link href={url} code>@tauri-store/shared</Link>.</li>
  </ul>
</Changelog.Dependencies>

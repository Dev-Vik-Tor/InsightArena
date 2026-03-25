import {
  IsOptional,
  IsString,
  IsUrl,
  Matches,
  MaxLength,
  MinLength,
} from 'class-validator';
import { ApiPropertyOptional } from '@nestjs/swagger';

export class UpdateUserDto {
  @ApiPropertyOptional({
    description: 'Display name (alphanumeric, 3–30 chars)',
    example: 'StellarTrader42',
  })
  @IsOptional()
  @IsString()
  @MinLength(3, { message: 'username must be at least 3 characters' })
  @MaxLength(30, { message: 'username must be at most 30 characters' })
  @Matches(/^[a-zA-Z0-9_]+$/, {
    message: 'username must be alphanumeric (letters, numbers, underscores)',
  })
  username?: string;

  @ApiPropertyOptional({
    description: 'Profile avatar URL',
    example: 'https://example.com/avatar.png',
  })
  @IsOptional()
  @IsString()
  @IsUrl({}, { message: 'avatar_url must be a valid URL' })
  avatar_url?: string;
}
